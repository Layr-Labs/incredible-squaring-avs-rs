use crate::error::OperatorError::{self, ECDSAKeystoreSigner};
use alloy::{
    primitives::{address, keccak256, Address},
    providers::WsConnect,
    rpc::types::Filter,
    signers::local::LocalSigner,
    sol_types::{SolEvent, SolValue},
};
use std::fs;
use std::path::PathBuf;

use alloy_provider::{Provider, ProviderBuilder};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::BlsKeypair;
use eigen_crypto_keystore::EncodedKeystore;
use eigen_types::operator::OperatorId;
use eyre::Result;
use futures_util::stream::StreamExt;
use incredible_aggregator::SignedTaskResponse;
use incredible_bindings::IncredibleSquaringTaskManager::{self, NewTaskCreated, TaskResponse};
use incredible_config::IncredibleConfig;
use rust_bls_bn254::sign;

/// Main Operator
#[derive(Debug)]
pub struct OperatorBuilder {
    rpc_url: String,

    operator_addr: Address,

    key_pair: BlsKeypair,

    operator_id: OperatorId,
}

impl OperatorBuilder {
    /// Build the Operator Builder
    pub fn build(config: IncredibleConfig) -> Result<Self, OperatorError> {
        // Read ECDSA private key from path

        println!("key store path file : {:?}", config.ecdsa_keystore_path());
        let signer_result = LocalSigner::decrypt_keystore(
            config.ecdsa_keystore_path(),
            config.ecdsa_keystore_password(),
        );

        match signer_result {
            Ok(signer) => {
                println!("signer : {signer:?}");
                let path = PathBuf::from(config.bls_keystore_path());
                let contents =
                    fs::read_to_string(path).expect("Should have been able to read the file");

                let encoded_keystore_result =
                    EncodedKeystore::from_string(contents, Some(config.bls_keystore_password()));

                match encoded_keystore_result {
                    Ok(encoded_keystore) => {
                        let bls_keypair = encoded_keystore
                            .into_bls_keypair()
                            .expect("failed to convert keystore into blskeypair");

                        Ok(Self {
                            rpc_url: config.rpc_url(),
                            operator_addr: config.operator_address(),
                            key_pair: bls_keypair,
                            operator_id: config.operator_id(),
                        });
                    }
                    Err(_) => Err(OperatorError::EncodedKeystore),
                }
            }
            Err(e) => {
                println!("Error is {e:?}");
                Err(ECDSAKeystoreSigner)
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
