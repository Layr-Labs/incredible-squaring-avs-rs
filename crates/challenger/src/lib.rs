//! Challenger crate
use alloy::consensus::Transaction;
use eigen_utils::{get_provider, get_ws_provider};
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    Task, TaskResponse, TaskResponseMetadata,
};
use incredible_bindings::incrediblesquaringtaskmanager::BN254::G1Point;
use std::collections::HashMap;
/// Challenger Error
pub mod error;
mod fake_challenger;
use alloy::providers::Provider;
use alloy::rpc::types::{BlockNumberOrTag, Filter};
use alloy::rpc::types::{Log, TransactionReceipt};
use alloy::sol_types::{SolCall, SolEvent};
use error::ChallengerError;
use eyre::Result;
use futures_util::stream::StreamExt;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::{
    respondToTaskCall, NewTaskCreated, TaskResponded,
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
    /// Get tasks
    pub fn tasks(&self) -> &HashMap<u32, Task> {
        &self.tasks
    }

    /// Start the challenger
    pub async fn start_challenger(&mut self) -> Result<()> {
        info!("challenger crate launched");

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

        loop {
            tokio::select! {
                Some(log) = task_responded_stream.next() => {
                    let task_index = self.process_task_response_log(log).await?;
                    if self.tasks.contains_key(&task_index) {
                        self.call_challenge(task_index).await?;
                    }
                },
                Some(log) = new_task_created_stream.next() => {
                    info!("challenger: picked up a new task ");
                    let new_task_created_option = log.log_decode::<NewTaskCreated>().ok();

                    if let Some(data) = new_task_created_option {
                        let m = data.data();
                        let new_task_cr = NewTaskCreated {
                            taskIndex: m.taskIndex,
                            task: m.task.clone(),
                        };

                        let _ = self.process_new_task_created_log(new_task_cr);

                    }
                },
                else => {
                    // If both streams are exhausted, break the loop.
                    info!("challenger:No more logs to process, exiting loop.");
                    break;
                }
            };
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
                    info!("calling raise challenge");
                    let _ = self.raise_challenge(task_index).await;
                    return Ok(());
                }
                info!("challenger:correct answer, no challenge raised");
                Ok(())
            } else {
                Err(ChallengerError::TaskResponseNotFound)
            }
        } else {
            Err(ChallengerError::TaskNotFound)
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
                            let calldata = transaction_data.inner.input();
                            let decoded = respondToTaskCall::abi_decode(calldata, false);

                            match decoded {
                                Ok(decoded) => {
                                    let non_signer_stakes_and_signature =
                                        decoded.nonSignerStakesAndSignature;

                                    let non_signing_operator_pub_keys: Vec<G1Point> =
                                        non_signer_stakes_and_signature
                                            .nonSignerPubkeys
                                            .iter()
                                            .map(|pub_key| G1Point {
                                                X: pub_key.X,
                                                Y: pub_key.Y,
                                            })
                                            .collect();
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

    use crate::fake_challenger::FakeChallenger;

    use super::*;
    use alloy::{
        hex::FromHex,
        primitives::{Bytes, FixedBytes, TxHash, U256},
    };
    use incredible_chainio::fake_avs_writer::FakeAvsWriter;
    use incredible_task_generator::TaskManager;
    use incredible_testing_utils::{
        get_incredible_squaring_operator_state_retriever,
        get_incredible_squaring_registry_coordinator, get_incredible_squaring_task_manager,
    };
    use std::str::FromStr;
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
keystore_2_path = "../testing-utils/src/bls_keystore_2.json"
keystore_2_password = "test"

[operator_config]
operator_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
operator_id = "0xb345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"
operator_2_address = "0x0b065a0423f076a340f37e16e1ce22e23d66caf2"
operator_2_id = "0x17a0935b43b64cc3536d48621208fddb680ef8998561f0a1669a3ccda66676be"
[task_manager_config]
signer = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"
"#;

    /// Build challenger
    pub(crate) async fn build_challenger() -> Result<Challenger, ChallengerError> {
        let mut config: IncredibleConfig = toml::from_str(INCREDIBLE_CONFIG_FILE)?;
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
        Challenger::build(config).await
    }

    #[tokio::test]
    async fn test_process_new_task_created_log() {
        let mut challenger = build_challenger().await.unwrap();
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

    #[tokio::test]
    async fn test_call_challenge() {
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

        let fake_avs_writer = FakeAvsWriter {
            task_manager_addr: get_incredible_squaring_task_manager().await,
            signer: config.get_signer(),
            rpc_url: config.http_rpc_url(),
        };
        let mut fake_challenger = FakeChallenger {
            fake_avs_writer: fake_avs_writer.clone(),
            ws_url: config.ws_rpc_url(),
            rpc_url: config.http_rpc_url(),
            tasks: HashMap::new(),
            task_responses: HashMap::new(),
        };

        let task_manager = TaskManager::new(
            get_incredible_squaring_task_manager().await,
            config.http_rpc_url(),
            config.task_manager_signer(),
        );

        let receipt = task_manager.create_new_task(U256::from(2)).await.unwrap();

        let log = receipt.inner.logs().first().unwrap();
        let new_task_created_log = log.log_decode::<NewTaskCreated>().unwrap();
        let NewTaskCreated { taskIndex, task } = new_task_created_log.data();

        fake_challenger.tasks.insert(*taskIndex, task.clone());

        let task_response = TaskResponseData {
            task_response: TaskResponse {
                referenceTaskIndex: *taskIndex,
                numberSquared: task.numberToBeSquared * task.numberToBeSquared,
            },
            task_response_metadata: TaskResponseMetadata {
                taskResponsedBlock: task.taskCreatedBlock,
                hashOfNonSigners: FixedBytes::default(),
            },
            non_signing_operator_pub_keys: vec![],
        };
        fake_challenger
            .task_responses
            .insert(*taskIndex, task_response);

        let hash = fake_challenger.raise_challenge(*taskIndex).await.unwrap();
        assert_eq!(hash, TxHash::default()); // assert that it actually called raise challenge to contract by getting a default tx hash from our fake avs writer
    }

    #[tokio::test]
    pub(crate) async fn test_process_task_response_log() {
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

        let fake_avs_writer = FakeAvsWriter {
            task_manager_addr: get_incredible_squaring_task_manager().await,
            signer: config.get_signer(),
            rpc_url: config.http_rpc_url(),
        };
        let mut fake_challenger = FakeChallenger {
            fake_avs_writer: fake_avs_writer.clone(),
            ws_url: config.ws_rpc_url(),
            rpc_url: config.http_rpc_url(),
            tasks: HashMap::new(),
            task_responses: HashMap::new(),
        };

        let task_response_log= Log { inner: alloy::primitives::Log::new("0x22753e4264fddc6181dc7cce468904a80a363e44".parse().unwrap(), [FixedBytes::from_hex("0x349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a").unwrap()].to_vec(), Bytes::from_hex("0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000006cb2f2c2ccf43574f9e119f4860fd0f1b6036a43ad9db8a428ea09a4ee385112f3").unwrap()).unwrap(), block_hash: Some(FixedBytes::from_hex("0xc9781943aedf7d3040c117b515b9e94af34e564976cf4ddd309a1febfcf4fdb8").unwrap()), block_number: Some(108), block_timestamp: None, transaction_hash: Some(FixedBytes::from_hex("0x7c26ddc3ed0f8ce05be3c5046fd72e7d3493b4e08ee33d03c8d791621183ee55").unwrap()), transaction_index: Some(0), log_index: Some(0), removed: false};

        fake_challenger
            .process_task_response_log(task_response_log)
            .await
            .unwrap();

        assert_eq!(
            fake_challenger
                .task_responses
                .get(&1)
                .unwrap()
                .task_response
                .numberSquared,
            U256::from(1)
        );
    }
}
