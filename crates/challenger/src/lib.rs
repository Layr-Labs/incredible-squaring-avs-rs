//! Challenger crate
use eigen_utils::{get_provider, get_ws_provider};
use std::collections::HashMap;
/// Challenger Error
pub mod error;

use alloy::rpc::types::{BlockNumberOrTag, Filter};
use alloy::rpc::types::{Log, TransactionReceipt};
use alloy::sol_types::{SolCall, SolEvent};
use alloy_provider::Provider;
use error::ChallengerError;
use eyre::Result;
use futures_util::stream::StreamExt;
use incredible_bindings::IncredibleSquaringTaskManager::{
    respondToTaskCall, G1Point, NewTaskCreated, Task, TaskResponded, TaskResponse,
    TaskResponseMetadata,
};
use incredible_chainio::AvsWriter;
use incredible_config::IncredibleConfig;
use tracing::info;

/// Task Response Data
#[derive(Debug)]
pub struct TaskResponseData {
    task_response: TaskResponse,
    task_response_metadata: TaskResponseMetadata,
    non_signing_operator_pub_keys: Vec<G1Point>,
}

/// Main Challenger struct
#[derive(Debug)]
pub struct Challenger {
    avs_writer: AvsWriter,

    ws_url: String,

    rpc_url: String,

    tasks: HashMap<u32, Task>,

    task_responses: HashMap<u32, TaskResponseData>,
}

impl Challenger {
    /// New instance of Challenger
    pub async fn build(config: IncredibleConfig) -> Result<Self, ChallengerError> {
        let registry_coordinator_address = config.registry_coordinator_addr()?;
        let avs_writer = AvsWriter::new(
            registry_coordinator_address,
            config.http_rpc_url(),
            config.get_signer(),
        )
        .await?;
        Ok(Self {
            avs_writer,
            ws_url: config.ws_rpc_url(),
            rpc_url: config.http_rpc_url(),
            tasks: HashMap::new(),
            task_responses: HashMap::new(),
        })
    }

    pub fn tasks(&self) -> &HashMap<u32, Task> {
        &self.tasks
    }

    /// Start the challenger
    pub async fn start_challenger(&mut self) -> Result<()> {
        info!("Starting Challenger.");
        info!("Subscribed to new tasks");

        let wa = get_ws_provider(&self.ws_url).await?;

        let new_task_created_filter = Filter::new()
            .event("NewTaskCreated(uint32,(uint256,uint32,bytes,uint32))")
            .from_block(BlockNumberOrTag::Latest);
        let new_task_created_sub = wa.subscribe_logs(&new_task_created_filter).await?;

        let mut new_task_created_stream = new_task_created_sub.into_stream();

        let new_task_created_log = NewTaskCreated::SIGNATURE_HASH;

        let task_responded_filter = Filter::new()
            .event("TaskResponded((uint32,uint256),(uint32,bytes32))")
            .from_block(BlockNumberOrTag::Latest);
        let task_responded_sub = wa.subscribe_logs(&task_responded_filter).await?;

        let mut task_responded_stream = task_responded_sub.into_stream();

        let task_responded_log = TaskResponded::SIGNATURE_HASH;

        loop {
            let log = tokio::select! {
                Some(log) = task_responded_stream.next() => {
                    log
                },
                Some(log) = new_task_created_stream.next() => {
                    log
                },
                else => {
                    // If both streams are exhausted, break the loop.
                    info!("No more logs to process, exiting loop.");
                    break;
                }
            };

            let topic = log.topic0();

            if let Some(tp) = topic {
                if *tp == new_task_created_log {
                    info!("challenger picked up a new task ");
                    println!("challengerlog {:?}", log);
                    let new_task_created_option = log.log_decode::<NewTaskCreated>().ok();

                    if let Some(data) = new_task_created_option {
                        let m = data.data();
                        let new_task_cr = NewTaskCreated {
                            taskIndex: m.taskIndex,
                            task: m.task.clone(),
                        };

                        let t_index = self.process_new_task_created_log(new_task_cr);

                        if let Some(_) = self.task_responses.get(&t_index) {
                            let call_c_result = self.call_challenge(t_index).await;
                            match call_c_result {
                                Ok(_) => continue,
                                Err(e) => {
                                    return Err(e.into());
                                }
                            }
                        }
                    }
                } else if *tp == task_responded_log {
                    info!(
                        "Task response log received by challenger {:?}",
                        task_responded_log
                    );

                    let task_index_result = self.process_task_response_log(log).await;

                    match task_index_result {
                        Ok(task_index) => {
                            if let Some(_) = self.tasks.get(&task_index) {
                                let call_c_result = self.call_challenge(task_index).await;
                                match call_c_result {
                                    Ok(_) => continue,
                                    Err(e) => {
                                        return Err(e.into());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Process new task created log
    pub fn process_new_task_created_log(&mut self, new_task_created_log: NewTaskCreated) -> u32 {
        self.tasks
            .insert(new_task_created_log.taskIndex, new_task_created_log.task);

        new_task_created_log.taskIndex
    }

    /// Call challenge
    pub async fn call_challenge(&self, task_index: u32) -> Result<(), ChallengerError> {
        if let Some(number_to_be_squared) = self.tasks.get(&task_index) {
            let num_to_square = number_to_be_squared.numberToBeSquared;
            println!(
                "task response for index {:?} {:?}",
                task_index,
                self.task_responses.get(&task_index)
            );
            if let Some(answer_in_response) = self.task_responses.get(&task_index) {
                let answer = answer_in_response.task_response.numberSquared;

                if answer != (num_to_square * num_to_square) {
                    let _ = self.raise_challenge(task_index).await;
                    return Ok(());
                }
                info!("task response is correct, no challenge");
                Ok(())
            } else {
                return Err(ChallengerError::TaskResponseNotFound);
            }
        } else {
            return Err(ChallengerError::TaskNotFound);
        }
    }

    /// Raise challenge
    pub async fn raise_challenge(
        &self,
        task_index: u32,
    ) -> Result<TransactionReceipt, ChallengerError> {
        let raise_challenge_result = self
            .avs_writer
            .raise_challenge(
                self.tasks[&task_index].clone(),
                self.task_responses[&task_index].task_response.clone(),
                self.task_responses[&task_index]
                    .task_response_metadata
                    .clone(),
                self.task_responses[&task_index]
                    .non_signing_operator_pub_keys
                    .clone(),
            )
            .await;
        match raise_challenge_result {
            Ok(raise_challenge) => Ok(raise_challenge),
            Err(e) => Err(error::ChallengerError::ChainIo(e)),
        }
    }

    /// Process task response log
    pub async fn process_task_response_log(
        &mut self,
        task_response_log: Log,
    ) -> Result<u32, ChallengerError> {
        let non_signing_operator_pub_keys_result = self
            .get_non_signing_operator_pub_keys(task_response_log.clone())
            .await;
        println!(
            "non signing operator pub keys result {:?}",
            non_signing_operator_pub_keys_result
        );
        match non_signing_operator_pub_keys_result {
            Ok(non_signing_operator_pub_key) => {
                let decoded_event = task_response_log.log_decode::<TaskResponded>().ok();
                println!("decoded event {:?}", decoded_event);
                if let Some(decoded) = decoded_event {
                    let data = decoded.data();

                    let task_response_data = TaskResponseData {
                        task_response: data.taskResponse.clone(),
                        task_response_metadata: data.taskResponseMetadata.clone(),
                        non_signing_operator_pub_keys: non_signing_operator_pub_key,
                    };

                    self.task_responses
                        .insert(data.taskResponse.referenceTaskIndex, task_response_data);
                    println!(
                        "process task response log for index {:?}  , number squared {:?}, {:?}",
                        data.taskResponse.referenceTaskIndex,
                        data.taskResponse.numberSquared,
                        task_response_log
                    );

                    Ok(data.taskResponse.referenceTaskIndex)
                } else {
                    Err(ChallengerError::DecodeEvent)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Get non signing operator pub keys
    pub async fn get_non_signing_operator_pub_keys(
        &self,
        log: Log,
    ) -> Result<Vec<G1Point>, ChallengerError> {
        let decoded_event = log.log_decode::<TaskResponded>().ok();
        println!("decoded event {:?}", decoded_event);
        if let Some(task_responded) = decoded_event {
            let tx_hash_result = task_responded.transaction_hash;
            if let Some(tx_hash) = tx_hash_result {
                let provider = get_provider(&self.rpc_url);

                let transaction_data_result = provider.get_transaction_by_hash(tx_hash).await;
                println!("tx hash res {:?}", transaction_data_result);
                match transaction_data_result {
                    Ok(transaction_data_option) => {
                        if let Some(transaction_data) = transaction_data_option {
                            let calldata = transaction_data.input;
                            println!("calldata {:?}", calldata);
                            let decoded = respondToTaskCall::abi_decode(&calldata, false);

                            match decoded {
                                Ok(decoded) => {
                                    let non_signer_stakes_and_signature =
                                        decoded.nonSignerStakesAndSignature;

                                    let mut non_signing_operator_pub_keys: Vec<G1Point> = vec![];

                                    for (i, pub_key) in non_signer_stakes_and_signature
                                        .nonSignerPubkeys
                                        .iter()
                                        .enumerate()
                                    {
                                        non_signing_operator_pub_keys[i] = G1Point {
                                            X: pub_key.X,
                                            Y: pub_key.Y,
                                        };
                                    }
                                    Ok(non_signing_operator_pub_keys)
                                }

                                Err(e) => Err(ChallengerError::AlloySolType(e)),
                            }
                        } else {
                            Err(ChallengerError::TaskResponseNotFound)
                        }
                    }
                    Err(e) => Err(ChallengerError::AlloyContractError(
                        alloy::contract::Error::TransportError(e),
                    )),
                }
            } else {
                Err(ChallengerError::TransactionHashNotFound)
            }
        } else {
            Err(ChallengerError::EmptyDecodedData)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use alloy::{
        hex::FromHex,
        primitives::{address, Bytes, FixedBytes, LogData, B256, U256},
        sol_types::SolValue,
    };
    use core::task;
    use eigen_utils::get_signer;
    use incredible_bindings::IncredibleSquaringTaskManager::{
        self, G2Point, NonSignerStakesAndSignature,
    };
    use incredible_task_generator::TaskManager;
    use incredible_testing_utils::{
        get_incredible_squaring_operator_state_retriever,
        get_incredible_squaring_registry_coordinator, get_incredible_squaring_strategy_address,
        get_incredible_squaring_task_manager,
    };
    use std::{io::Read, str::FromStr};
    const INCREDIBLE_CONFIG_FILE: &str = r#"
[rpc_config]
chain_id = 31337
http_rpc_url = "http://localhost:8545"
ws_rpc_url = "ws://localhost:8546"
signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

[ecdsa_config]
keystore_path = "../testing-utils/src/ecdsakeystore.json"
keystore_password = "test"

[bls_config]
keystore_path = "../testing-utils/src/blskeystore.json"
keystore_password = "testpassword"

[operator_config]
operator_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
operator_id = "0xb345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"

[task_manager_config]
signer = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"
"#;

    ///
    pub async fn build_challenger() -> Challenger {
        let mut config: IncredibleConfig = toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();
        config.set_registry_coordinator_addr(
            get_incredible_squaring_registry_coordinator()
                .await
                .to_string(),
        );
        config.set_operator_state_retriever(
            get_incredible_squaring_operator_state_retriever()
                .await
                .to_string(),
        );
        Challenger::build(config).await.unwrap()
    }

    #[tokio::test]
    async fn test_process_new_task_created_log() {
        let mut challenger = build_challenger().await;
        let new_task_created = NewTaskCreated {
            taskIndex: 1,
            task: Task {
                numberToBeSquared: U256::from(4),
                taskCreatedBlock: 105,
                quorumNumbers: Bytes::from_str("0x40").unwrap(),
                quorumThresholdPercentage: 5,
            },
        };
        challenger.process_new_task_created_log(new_task_created.clone());
        let task = challenger
            .tasks()
            .get(&new_task_created.clone().taskIndex)
            .unwrap();
        assert_eq!(
            task.numberToBeSquared,
            new_task_created.clone().task.numberToBeSquared
        );
        assert_eq!(
            task.taskCreatedBlock,
            new_task_created.clone().task.taskCreatedBlock
        );
        assert_eq!(
            task.quorumNumbers,
            new_task_created.clone().task.quorumNumbers
        );
        assert_eq!(
            task.quorumThresholdPercentage,
            new_task_created.clone().task.quorumThresholdPercentage
        );
    }
}
