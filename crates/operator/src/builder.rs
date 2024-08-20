use crate::error::OperatorError;
use alloy::{
    primitives::{keccak256, Address},
    providers::WsConnect,
    rpc::types::Filter,
    signers::{k256::ecdsa::SigningKey, local::LocalSigner},
    sol_types::{SolEvent, SolValue},
};
use alloy_provider::{Provider, ProviderBuilder};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_logger;
use eigen_types::operator::OperatorId;
use eyre::Result;
use futures_util::StreamExt;
use incredible_aggregator::SignedTaskResponse;
use incredible_bindings::IncredibleSquaringTaskManager::{self, NewTaskCreated, TaskResponse};
use incredible_config::IncredibleConfig;
use rust_bls_bn254::keystores::{
    base_keystore::Keystore, pbkdf2_keystore::Pbkdf2Keystore, scrypt_keystore::ScryptKeystore,
};

use crate::client::ClientAggregator;
use incredible_metrics::IncredibleMetrics;
use rand_core::OsRng;
use std::path::PathBuf;
use std::{env::temp_dir, fs};
use tracing::{debug, info};

/// Main Operator
#[derive(Debug)]
pub struct OperatorBuilder {
    http_rpc_url: String,

    ws_rpc_url: String,

    operator_addr: Address,

    key_pair: BlsKeyPair,

    operator_id: OperatorId,

    client: ClientAggregator,

    aggregator_ip_addr: String,

    metrics: IncredibleMetrics,

    signer: LocalSigner<SigningKey>,

    registry_coordinator: Address,

    operator_state_retriever: Address,
}

impl OperatorBuilder {
    /// Build the Operator Builder
    pub fn build(config: IncredibleConfig) -> Result<Self, OperatorError> {
        println!("opbuilder");
        // Read ECDSA private key from path
        let signer = LocalSigner::decrypt_keystore(
            config.ecdsa_keystore_path(),
            config.ecdsa_keystore_password(),
        )?;

        // Read BlsKey from path
        let keystore = Keystore::from_file(&config.bls_keystore_path())
            .unwrap()
            .decrypt(&config.bls_keystore_password())?;
        // TODO(supernova): Add this method in sdk in bls crate
        let fr_key: String = keystore.iter().map(|&value| value as u8 as char).collect();

        let key_pair = BlsKeyPair::new(fr_key)?;
        let metrics = IncredibleMetrics::new();
        let operator_id = config.get_operator_id()?;
        let registry_coordinator_addr = config.registry_coordinator_addr()?;
        let operator_statr_retriever_addr = config.operator_state_retriever_addr()?;
        println!("opop");
        let operator_address = config.operator_address()?;

        Ok(Self {
            http_rpc_url: config.http_rpc_url(),
            ws_rpc_url: config.ws_rpc_url(),
            operator_addr: operator_address,
            key_pair,
            operator_id: operator_id,
            client: ClientAggregator::new(config.aggregator_ip_addr()),
            metrics,
            aggregator_ip_addr: config.aggregator_ip_addr(),
            signer,
            registry_coordinator: registry_coordinator_addr,
            operator_state_retriever: operator_statr_retriever_addr,
        })
    }

    pub fn bls_key_pair(&self) -> BlsKeyPair {
        self.key_pair.clone()
    }

    /// Processes new task
    pub fn process_new_task(&self, new_task_created: NewTaskCreated) -> TaskResponse {
        let num_squared =
            new_task_created.task.numberToBeSquared * new_task_created.task.numberToBeSquared;

        let task_response = TaskResponse {
            referenceTaskIndex: new_task_created.taskIndex,
            numberSquared: num_squared,
        };
        task_response
    }

    /// Start the operator
    pub async fn start_operator(&self) -> Result<()> {
        println!("11{}", self.registry_coordinator);
        println!("22{}", self.operator_state_retriever);
        println!("33{}", self.http_rpc_url.clone());
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_logger(),
            self.registry_coordinator,
            self.operator_state_retriever,
            self.http_rpc_url.clone(),
        )
        .await
        .unwrap();

        println!("avs regitstry reader {:?}", avs_registry_reader);

        let is_registered = avs_registry_reader
            .is_operator_registered(self.operator_addr.clone())
            .await?;

        // if is_registered {
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
                self.metrics.increment_num_tasks_received();
                let task_response = self.process_new_task(new_task_created);
                let signed_task_response = self.sign_task_response(task_response)?;

                let _ = self
                    .client
                    .send_signed_task_response(signed_task_response)
                    .await;
            }
            // }
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
        info!("signed task response : {:?}", signed_task_response);
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
    use std::str::FromStr;
    use IncredibleSquaringTaskManager::Task;

    #[tokio::test]
    async fn test_bls_keystore() {
        let output_path = "../testing-utils/src/blskeystore.json";
        let mut config = IncredibleConfig::default();
        config.set_bls_keystore_path(output_path.to_string());
        config.set_bls_keystore_password("testpassword".to_string());
        let operator_builder = OperatorBuilder::build(config).unwrap();

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

        let incredible_config_file = r#"
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
        registry_coordinator_addr = "0x3aAde2dCD2Df6a8cAc689EE797591b2913658659"
        operator_state_retriever_addr = "0x276C216D241856199A83bf27b2286659e5b877D3"
        "#;

        let incredible_config: IncredibleConfig = toml::from_str(incredible_config_file).unwrap();

        println!("incredible config {:?}", incredible_config);
        let operator_builder = OperatorBuilder::build(incredible_config).unwrap();

        let task_response = operator_builder.process_new_task(new_task_created);

        assert_eq!(task_response.numberSquared, U256::from(16));
        assert_eq!(task_response.referenceTaskIndex, 1u32);
    }

    #[test]
    fn test_build_operator() {}

    #[test]
    fn test_sign_task_response() {}
}
