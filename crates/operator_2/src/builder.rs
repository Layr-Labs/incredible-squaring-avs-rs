use std::sync::Arc;

use crate::error::OperatorError;
use alloy::{
    primitives::{keccak256, Address},
    providers::WsConnect,
    rpc::types::Filter,
    sol_types::{SolEvent, SolValue},
};
use incredible_operator::client::ClientAggregator;

#[cfg(feature = "integration_tests")]
use alloy::primitives::U256;
use alloy_provider::{Provider, ProviderBuilder};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_client_eth::instrumented_client::InstrumentedClient;
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_logger;
use eigen_types::operator::OperatorId;
use eyre::Result;
use futures_util::StreamExt;
use incredible_aggregator::rpc_server::SignedTaskResponse;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::TaskResponse;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::{
    self, NewTaskCreated,
};
use incredible_config::IncredibleConfig;
use rust_bls_bn254::keystores::base_keystore::Keystore;
use tracing::info;

/// Main Operator
#[derive(Debug)]
pub struct OperatorBuilder {
    http_rpc_url: String,

    ws_rpc_url: String,

    operator_addr: Address,

    key_pair: BlsKeyPair,

    operator_id: OperatorId,

    client: Option<Arc<ClientAggregator>>,

    registry_coordinator: Address,

    operator_state_retriever: Address,
}

impl OperatorBuilder {
    /// Build the Operator Builder
    pub async fn build(
        config: IncredibleConfig,
        client: Option<Arc<ClientAggregator>>,
    ) -> Result<Self, OperatorError> {
        let _instrumented_client = InstrumentedClient::new(&config.http_rpc_url()).await;
        // Read BlsKey from path
        let keystore = Keystore::from_file(&config.bls_keystore_2_path())?
            .decrypt(&config.bls_keystore_2_password())?;

        // TODO(supernova): Add this method in sdk in bls crate
        let fr_key: String = keystore.iter().map(|&value| value as char).collect();
        let key_pair = BlsKeyPair::new(fr_key)?;
        let operator_id = config.get_operator_2_id()?;
        let registry_coordinator_addr = config.registry_coordinator_addr()?;
        let operator_statr_retriever_addr = config.operator_state_retriever_addr()?;
        let operator_address = config.operator_2_address()?;
        Ok(Self {
            http_rpc_url: config.http_rpc_url(),
            ws_rpc_url: config.ws_rpc_url(),
            operator_addr: operator_address,
            key_pair,
            operator_id,
            client,
            registry_coordinator: registry_coordinator_addr,
            operator_state_retriever: operator_statr_retriever_addr,
        })
    }

    /// Get the [`BlsKeyPair`]
    pub fn bls_key_pair(&self) -> BlsKeyPair {
        self.key_pair.clone()
    }

    /// Processes new task
    pub fn process_new_task(&self, new_task_created: NewTaskCreated) -> TaskResponse {
        #[allow(unused_mut)]
        #[allow(unused_assignments)]
        let mut number_to_be_squared = new_task_created.task.numberToBeSquared;

        #[cfg(feature = "integration_tests")]
        {
            number_to_be_squared = U256::from(9);
            info!("Challenger test: setting number to be squared to 9");
        }

        let num_squared = number_to_be_squared * number_to_be_squared;

        TaskResponse {
            referenceTaskIndex: new_task_created.taskIndex,
            numberSquared: num_squared,
        }
    }

    /// Start the operator
    pub async fn start_operator(&mut self) -> Result<()> {
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_logger(),
            self.registry_coordinator,
            self.operator_state_retriever,
            self.http_rpc_url.clone(),
        )
        .await
        .unwrap();
        let is_registered = avs_registry_reader
            .is_operator_registered(self.operator_addr)
            .await?;
        info!("is_operator2_registered {}", is_registered);
        if is_registered {
            info!("Starting operator");

            let ws = WsConnect::new(self.ws_rpc_url.clone());
            let provider = ProviderBuilder::new().on_ws(ws).await?;

            let filter = Filter::new().event_signature(NewTaskCreated::SIGNATURE_HASH);
            let sub = provider.subscribe_logs(&filter).await?;
            let mut stream = sub.into_stream();

            while let Some(log) = stream.next().await {
                let task_option = log
                    .log_decode::<IncredibleSquaringTaskManager::NewTaskCreated>()
                    .ok();
                if let Some(task) = task_option {
                    let data = task.data();
                    let new_task_created = NewTaskCreated {
                        task: data.task.clone(),
                        taskIndex: data.taskIndex,
                    };
                    info!(
                        "operator2 picked up a new task , index: {} ",
                        data.taskIndex
                    );
                    incredible_metrics::increment_num_tasks_received();
                    let task_response = self.process_new_task(new_task_created);
                    let signed_task_response = self.sign_task_response(task_response)?;
                    if let Some(client) = &self.client {
                        let _ = client.send_signed_task_response(signed_task_response).await;
                    }
                }
            }
        }
        Ok(())
    }

    /// Sign the task response
    pub fn sign_task_response(
        &self,
        task_response: TaskResponse,
    ) -> Result<SignedTaskResponse, OperatorError> {
        let encoded_response = TaskResponse::abi_encode(&task_response);
        let hash_msg = keccak256(encoded_response);

        let signed_msg = self.key_pair.sign_message(hash_msg.as_slice());
        let signed_task_response =
            SignedTaskResponse::new(task_response, signed_msg, self.operator_id);
        Ok(signed_task_response)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use alloy::primitives::Bytes;
    use alloy::primitives::U256;
    use ark_ec::AffineRepr;
    use ark_ff::PrimeField;
    use eigen_crypto_bn254::utils::verify_message;
    use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::Task;
    use incredible_testing_utils::{
        get_incredible_squaring_operator_state_retriever,
        get_incredible_squaring_registry_coordinator,
    };
    use std::str::FromStr;
    const INCREDIBLE_CONFIG_FILE: &str = r#"
    [rpc_config]
    chain_id = 31337
    http_rpc_url = "http://localhost:8545"
    ws_rpc_url = "ws://localhost:8545"
    signer = "0x337edbf6fef9af147f49c04c1004da47a8bf9f88c01022b7dd108e31c037f075"

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
    "#;

    #[tokio::test]
    async fn test_bls_keystore() {
        let mut incredible_config: IncredibleConfig =
            toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();
        incredible_config.set_registry_coordinator_addr(
            get_incredible_squaring_registry_coordinator()
                .await
                .to_string(),
        );
        incredible_config.set_operator_state_retriever(
            get_incredible_squaring_operator_state_retriever()
                .await
                .to_string(),
        );
        let operator_builder = OperatorBuilder::build(incredible_config, None)
            .await
            .unwrap();

        assert_eq!(
            U256::from_limbs(
                operator_builder
                    .key_pair
                    .public_key()
                    .g1()
                    .x()
                    .unwrap()
                    .into_bigint()
                    .0
            ),
            U256::from_str(
                "654664748928620715566514527065607787384626422829919343002201686008542704547"
            )
            .unwrap()
        );
    }

    #[tokio::test]
    async fn test_process_new_task() {
        let new_task_created = NewTaskCreated {
            taskIndex: 1,
            task: Task {
                numberToBeSquared: U256::from(4),
                taskCreatedBlock: 105,
                quorumNumbers: Bytes::from_str("0x40").unwrap(),
                quorumThresholdPercentage: 5,
            },
        };

        let mut incredible_config: IncredibleConfig =
            toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();
        incredible_config.set_registry_coordinator_addr(
            get_incredible_squaring_registry_coordinator()
                .await
                .to_string(),
        );
        incredible_config.set_operator_state_retriever(
            get_incredible_squaring_operator_state_retriever()
                .await
                .to_string(),
        );
        let operator_builder = OperatorBuilder::build(incredible_config, None)
            .await
            .unwrap();

        let task_response = operator_builder.process_new_task(new_task_created);

        assert_eq!(task_response.numberSquared, U256::from(16));
        assert_eq!(task_response.referenceTaskIndex, 1u32);
    }

    #[tokio::test]
    async fn test_build_operator() {
        let mut incredible_config: IncredibleConfig =
            toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();
        incredible_config.set_registry_coordinator_addr(
            get_incredible_squaring_registry_coordinator()
                .await
                .to_string(),
        );
        incredible_config.set_operator_state_retriever(
            get_incredible_squaring_operator_state_retriever()
                .await
                .to_string(),
        );
        let _ = OperatorBuilder::build(incredible_config, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_sign_task_response() {
        let task_response = TaskResponse {
            referenceTaskIndex: 1,
            numberSquared: U256::from(16),
        };

        let mut incredible_config: IncredibleConfig =
            toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();
        incredible_config.set_registry_coordinator_addr(
            get_incredible_squaring_registry_coordinator()
                .await
                .to_string(),
        );
        incredible_config.set_operator_state_retriever(
            get_incredible_squaring_operator_state_retriever()
                .await
                .to_string(),
        );
        let operator_builder = OperatorBuilder::build(incredible_config, None)
            .await
            .unwrap();
        let signed_task_response = operator_builder
            .sign_task_response(task_response.clone())
            .unwrap();

        let bls_key_pair = operator_builder.bls_key_pair();
        let encoded_response = TaskResponse::abi_encode(&task_response);
        let hash_msg = keccak256(encoded_response);
        assert!(verify_message(
            bls_key_pair.public_key_g2().g2(),
            hash_msg.as_slice(),
            signed_task_response.signature().g1_point().g1()
        ));
    }
}
