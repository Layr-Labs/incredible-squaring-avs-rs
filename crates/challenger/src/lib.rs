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

        match non_signing_operator_pub_keys_result {
            Ok(non_signing_operator_pub_key) => {
                let decoded_event = task_response_log.log_decode::<TaskResponded>().ok();

                if let Some(decoded) = decoded_event {
                    let data = decoded.data();

                    let task_response_data = TaskResponseData {
                        task_response: data.taskResponse.clone(),
                        task_response_metadata: data.taskResponseMetadata.clone(),
                        non_signing_operator_pub_keys: non_signing_operator_pub_key,
                    };

                    self.task_responses
                        .insert(data.taskResponse.referenceTaskIndex, task_response_data);
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

        if let Some(task_responded) = decoded_event {
            let tx_hash_result = task_responded.transaction_hash;
            if let Some(tx_hash) = tx_hash_result {
                let provider = get_provider(&self.rpc_url);

                let transaction_data_result = provider.get_transaction_by_hash(tx_hash).await;

                match transaction_data_result {
                    Ok(transaction_data_option) => {
                        if let Some(transaction_data) = transaction_data_option {
                            let calldata = transaction_data.input;
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

    use std::str::FromStr;

    use super::*;
    use alloy::primitives::{Bytes, U256};

    const INCREDIBLE_CONFIG_FILE: &str = r#"
[rpc_config]
chain_id = 31337
http_rpc_url = "http://localhost:8545"
ws_rpc_url = "ws://localhost:8546"
signer = "0x337edbf6fef9af147f49c04c1004da47a8bf9f88c01022b7dd108e31c037f075"

[ecdsa_config]
keystore_path = "../testing-utils/src/ecdsakeystore.json"
keystore_password = "test"

[bls_config]
keystore_path = "../testing-utils/src/blskeystore.json"
keystore_password = "testpassword"

[operator_config]
operator_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
operator_id = "0x0202020202020202020202020202020202020202020202020202020202020202"

[el_config]
registry_coordinator_addr = "0x276c216d241856199a83bf27b2286659e5b877d3"
operator_state_retriever_addr = "0x3aAde2dCD2Df6a8cAc689EE797591b2913658659"
"#;

    #[tokio::test]
    async fn test_process_new_task_created_log() {
        let config: IncredibleConfig = toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();

        let mut challenger = Challenger::build(config).await.unwrap();
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
