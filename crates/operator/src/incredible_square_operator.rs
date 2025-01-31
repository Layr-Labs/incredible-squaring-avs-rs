use alloy::primitives::Address;
use eigen_aggregator::traits::TaskResponse;
use eigen_client_eth::instrumented_client::InstrumentedClient;
use eigen_crypto_bls::BlsKeyPair;
use eigen_operator::traits::Operator;
use eigen_operator::{client::ClientAggregator, error::OperatorError};
use eigen_types::avs::TaskIndex;
use eigen_types::operator::OperatorId;
use eyre::Result;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::TaskResponse as SolTaskResponse;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use incredible_config::IncredibleConfig;
use rust_bls_bn254::keystores::base_keystore::Keystore;
use serde::{Deserialize, Serialize};

/// Main Operator
#[derive(Debug)]
pub struct IncredibleSquareOperator {
    /// HTTP RPC URL
    pub http_rpc_url: String,
    /// WS RPC URL
    pub ws_rpc_url: String,
    /// Operator Address
    pub operator_addr: Address,
    /// [`BlsKeyPair`]
    pub key_pair: BlsKeyPair,
    /// Operator ID
    pub operator_id: OperatorId,
    /// [`ClientAggregator`]
    pub client: ClientAggregator,
    /// Registry Coordinator Address
    pub registry_coordinator: Address,
    /// Operator State Retriever Address
    pub operator_state_retriever: Address,
}

#[derive(Debug, Serialize, Deserialize)]
/// Task Response for Incredible Square Operator
pub struct IncredibleTaskResponse(SolTaskResponse);

impl TaskResponse for IncredibleTaskResponse {
    fn task_index(&self) -> TaskIndex {
        self.0.referenceTaskIndex
    }
    fn digest(&self) -> alloy::primitives::B256 {
        todo!()
    }
}

impl Operator for IncredibleSquareOperator {
    type TaskResponse = IncredibleTaskResponse;
    type NewTaskEvent = NewTaskCreated;

    fn process_new_task(new_task_created: NewTaskCreated) -> Self::TaskResponse {
        #[allow(unused_mut)]
        #[allow(unused_assignments)]
        let mut number_to_be_squared = new_task_created.task.numberToBeSquared;

        #[cfg(feature = "integration_tests")]
        {
            use tracing::info;

            number_to_be_squared = alloy::primitives::U256::from(9);
            info!("Challenger test: setting number to be squared to 9");
        }

        let num_squared = number_to_be_squared * number_to_be_squared;

        IncredibleTaskResponse(SolTaskResponse {
            referenceTaskIndex: new_task_created.taskIndex,
            numberSquared: num_squared,
        })
    }
}

impl IncredibleSquareOperator {
    /// Build the Operator Builder
    pub async fn new(config: IncredibleConfig) -> Result<Self, OperatorError> {
        let _instrumented_client = InstrumentedClient::new(&config.http_rpc_url()).await;
        // Read BlsKey from path
        let keystore = Keystore::from_file(&config.bls_keystore_path())
            .map_err(|_| OperatorError::RegistrationError)? // TODO: change error type
            .decrypt(&config.bls_keystore_password())
            .map_err(|_| OperatorError::RegistrationError)?; // TODO: change error type

        // TODO(supernova): Add this method in sdk in bls crate
        let fr_key: String = keystore.iter().map(|&value| value as char).collect();
        let key_pair = BlsKeyPair::new(fr_key)?;
        let operator_id = config
            .get_operator_id()
            .map_err(|_| OperatorError::RegistrationError)?; // TODO: change error type
        let registry_coordinator_addr = config
            .registry_coordinator_addr()
            .map_err(|_| OperatorError::RegistrationError)?; // TODO: change error type
        let operator_statr_retriever_addr = config
            .operator_state_retriever_addr()
            .map_err(|_| OperatorError::RegistrationError)?; // TODO: change error type
        let operator_address = config
            .operator_address()
            .map_err(|_| OperatorError::RegistrationError)?; // TODO change error type
        let mut client = ClientAggregator::new(config.aggregator_ip_addr());
        let _ = client.dial_aggregator_rpc_client();
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{keccak256, Bytes, U256},
        sol_types::SolValue,
    };
    use ark_ec::AffineRepr;
    use ark_ff::fields::PrimeField;
    use eigen_crypto_bn254::utils::verify_message;
    use eigen_operator::traits::Operator;
    use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::Task;
    use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::TaskResponse;
    use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
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
        let operator_builder = IncredibleSquareOperator::new(incredible_config)
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
                "277950648056014144722774518899051149098728246263316284984520891067822832300"
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

        let task_response = IncredibleSquareOperator::process_new_task(new_task_created);

        assert_eq!(task_response.0.numberSquared, U256::from(16));
        assert_eq!(task_response.0.referenceTaskIndex, 1u32);
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
        let _ = IncredibleSquareOperator::new(incredible_config)
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
        let operator_builder = IncredibleSquareOperator::new(incredible_config)
            .await
            .unwrap();
        let signed_task_response = IncredibleSquareOperator::sign_task_response(
            &operator_builder.key_pair,
            &operator_builder.operator_id,
            super::IncredibleTaskResponse(task_response.clone()),
        )
        .unwrap();

        let bls_key_pair = &operator_builder.key_pair;
        let encoded_response = TaskResponse::abi_encode(&task_response);
        let hash_msg = keccak256(encoded_response);
        assert!(verify_message(
            bls_key_pair.public_key_g2().g2(),
            &hash_msg,
            signed_task_response.signature().g1_point().g1()
        ));
    }
}
