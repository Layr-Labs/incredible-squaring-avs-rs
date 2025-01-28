use std::sync::Arc;

use alloy::{primitives::Address, providers::WsConnect, rpc::types::Filter, sol_types::SolEvent};
use alloy_provider::{Provider, ProviderBuilder};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_client_eth::instrumented_client::InstrumentedClient;
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_logger;
use eigen_operator::{client::ClientAggregator, error::OperatorError, operator};
use eigen_types::operator::OperatorId;
use eyre::Result;
use futures_util::StreamExt;
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
            number_to_be_squared = alloy::primitives::U256::from(9);
            info!("Challenger test: setting number to be squared to 9");
        }

        let num_squared = number_to_be_squared * number_to_be_squared;

        TaskResponse {
            referenceTaskIndex: new_task_created.taskIndex,
            numberSquared: num_squared,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use alloy::primitives::{keccak256, Bytes, U256};
    use alloy::sol_types::SolValue;
    use ark_ec::AffineRepr;
    use ark_ff::PrimeField;
    use eigen_crypto_bn254::utils::verify_message;
    use eigen_operator::operator;
    use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::Task;
    use incredible_testing_utils::{
        get_incredible_squaring_operator_state_retriever,
        get_incredible_squaring_registry_coordinator,
    };
    use std::fs;
    use std::path::Path;
    use std::str::FromStr;

    fn get_config_path() -> String {
        let config_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("operator/operator_test_config.toml");
        fs::read_to_string(config_path).unwrap()
    }

    #[tokio::test]
    async fn test_bls_keystore() {
        let mut incredible_config: IncredibleConfig = toml::from_str(&get_config_path()).unwrap();
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

        let mut incredible_config: IncredibleConfig = toml::from_str(&get_config_path()).unwrap();
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
        let mut incredible_config: IncredibleConfig = toml::from_str(&get_config_path()).unwrap();
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

        let mut incredible_config: IncredibleConfig = toml::from_str(&get_config_path()).unwrap();
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
        let signed_task_response = operator::sign_task_response(
            &operator_builder.key_pair,
            &operator_builder.operator_id,
            task_response.clone(),
        )
        .unwrap();

        let bls_key_pair = operator_builder.bls_key_pair();
        let encoded_response = TaskResponse::abi_encode(&task_response);
        let hash_msg = keccak256(encoded_response);
        assert!(verify_message(
            bls_key_pair.public_key_g2().g2(),
            &hash_msg,
            signed_task_response.signature().g1_point().g1()
        ));
    }
}
