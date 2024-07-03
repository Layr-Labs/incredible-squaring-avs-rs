use crate::error::OperatorError::{self, ECDSAKeystoreSigner};
use alloy::{
    primitives::{address, keccak256, Address},
    providers::WsConnect,
    rpc::types::Filter,
    signers::local::LocalSigner,
    sol_types::{SolEvent, SolValue},
};
use alloy_provider::{Provider, ProviderBuilder};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::attestation::KeyPair;
use eigen_types::operator::OperatorId;
use eyre::Result;
use futures_util::stream::StreamExt;
use incredible_aggregator::SignedTaskResponse;
use incredible_bindings::IncredibleSquaringTaskManager::{self, NewTaskCreated, TaskResponse};
use incredible_config::IncredibleConfig;
use rust_bls_bn254::sign;

/// Main Operator
#[derive(Debug, Default)]
pub struct OperatorBuilder {
    rpc_url: String,

    operator_addr: Address,

    key_pair: KeyPair,

    operator_id: OperatorId,
}

impl OperatorBuilder {
    /// Build the Operator Builder
    pub fn build(&self, config: IncredibleConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Read ECDSA private key from path

        println!("key store path file : {:?}", config.ecdsa_keystore_path());
        let signer_result = LocalSigner::decrypt_keystore(
            config.ecdsa_keystore_path(),
            config.ecdsa_keystore_password(),
        );

        match signer_result {
            Ok(signer) => {
                println!("signer : {signer:?}");

                // TODO Bls keystore
                // 2335, 2333, 2334 eips for bls keystore wip

                todo!()
            }
            Err(e) => {
                println!("Error is {e:?}");
                Err(Box::new(ECDSAKeystoreSigner))
            }
        }
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
        let registry_coordinator = address!();
        let operator_state_retriever = address!();
        let avs_registry_reader = AvsRegistryChainReader::new(
            registry_coordinator,
            operator_state_retriever,
            self.rpc_url.clone(),
        )
        .await
        .expect("avs registry reader failed to build");

        let is_registered_result = avs_registry_reader
            .is_operator_registered(self.operator_addr)
            .await;

        if is_registered_result.is_ok() {
            // todo tracing
            println!("Starting operator");

            let ws = WsConnect::new(self.rpc_url.clone());
            let provider = ProviderBuilder::new().on_ws(ws).await?;

            let filter = Filter::new().event(NewTaskCreated::SIGNATURE);
            let sub = provider.subscribe_logs(&filter).await?;
            let mut stream = sub.into_stream();

            while let Some(log) = stream.next().await {
                println!("logs : {log:?}");
                let task_option = log
                    .log_decode::<IncredibleSquaringTaskManager::NewTaskCreated>()
                    .ok();

                if let Some(task) = task_option {
                    let data = task.data();
                    let new_task_created = NewTaskCreated {
                        task: data.task.clone(),
                        taskIndex: data.taskIndex,
                    };
                    let task_response = self.process_new_task(new_task_created);
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

        let signed_msg_result = sign(self.key_pair.priv_key(), hash_msg.as_slice());

        match signed_msg_result {
            Ok(signature) => {
                let signed_task_response =
                    SignedTaskResponse::new(task_response, signature, self.operator_id);

                Ok(signed_task_response)
            }
            Err(_) => Err(OperatorError::SignUsingBlsKeyPair),
        }
    }
}
