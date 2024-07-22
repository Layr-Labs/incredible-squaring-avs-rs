use crate::error::OperatorError::{self, ECDSAKeystoreSigner};
use alloy::{
    primitives::{keccak256, Address},
    providers::WsConnect,
    rpc::types::Filter,
    signers::{k256::ecdsa::SigningKey, local::LocalSigner},
    sol_types::{SolEvent, SolValue},
};
use alloy_provider::{Provider, ProviderBuilder};
use eigen_client_avsregistry::{error::AvsRegistryError, reader::AvsRegistryChainReader};
use eigen_crypto_bls::BlsKeypair;
use eigen_crypto_keystore::EncodedKeystore;
use eigen_types::operator::OperatorId;
use eyre::Result;
use futures_util::stream::StreamExt;
use incredible_aggregator::SignedTaskResponse;
use incredible_bindings::IncredibleSquaringTaskManager::{self, NewTaskCreated, TaskResponse};
use incredible_config::IncredibleConfig;
use rust_bls_bn254::sign;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info};

use crate::client::ClientAggregator;
use incredible_metrics::IncredibleMetrics;

/// Main Operator
#[derive(Debug)]
pub struct OperatorBuilder {
    rpc_url: String,

    operator_addr: Address,

    key_pair: BlsKeypair,

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
        // Read ECDSA private key from path

        let signer_result = LocalSigner::decrypt_keystore(
            config.ecdsa_keystore_path(),
            config.ecdsa_keystore_password(),
        );

        match signer_result {
            Ok(signer) => {
                println!("signer key{:?} ",signer.address());
                let path = PathBuf::from(config.bls_keystore_path());
                let contents_result = fs::read_to_string(path);

                match contents_result {
                    Ok(contents) => {
                        let encoded_keystore_result = EncodedKeystore::from_string(
                            contents,
                            Some(config.bls_keystore_password()),
                        );

                        match encoded_keystore_result {
                            Ok(encoded_keystore) => {
                                let bls_keypair = encoded_keystore
                                    .into_bls_keypair()
                                    .expect("failed to convert keystore into blskeypair");
                                let metrics = IncredibleMetrics::new();

                                let operator_id_result = config.get_operator_id();
                                match operator_id_result {
                                    Ok(operator_id) => {
                                        let registry_coordinator_addr_result =
                                            config.registry_coordinator_addr();
                                        let operator_statr_retriever_addr_result =
                                            config.operator_state_retriever_addr();

                                        match registry_coordinator_addr_result {
                                            Ok(registry_coordinator_addr) => {
                                                match operator_statr_retriever_addr_result {
                                                    Ok(operator_statr_retriever_addr) => {
                                                        let operator_address_result =
                                                            config.operator_address();

                                                        match operator_address_result {
                                                            Ok(operator_address) => {
                                                                
                                                                Ok(Self {
                                                                rpc_url: config.get_rpc_url(),
                                                                operator_addr: operator_address,
                                                                key_pair: bls_keypair,
                                                                operator_id: operator_id,
                                                                client: ClientAggregator::new(
                                                                    config.aggregator_ip_addr(),
                                                                ),
                                                                metrics,
                                                                aggregator_ip_addr: config
                                                                    .aggregator_ip_addr(),
                                                                signer,
                                                                registry_coordinator:
                                                                    registry_coordinator_addr,
                                                                operator_state_retriever:
                                                                    operator_statr_retriever_addr,
                                                            })},

                                                            Err(e) => Err(
                                                                OperatorError::ConfigParseError(e),
                                                            ),
                                                        }
                                                    }

                                                    Err(e) => {
                                                        return Err(
                                                            OperatorError::ConfigParseError(e),
                                                        )
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                return Err(OperatorError::ConfigParseError(e));
                                            }
                                        }
                                    }
                                    Err(e) => Err(OperatorError::ConfigParseError(e)),
                                }
                            }
                            Err(_) => {
                                if config.get_no_bls_bool() {
                                    let metrics = IncredibleMetrics::new();
                                    
                                    let operator_id_result = config.get_operator_id();
                                    
                                    match operator_id_result {
                                        Ok(operator_id) => {
                                            let registry_coordinator_addr_result =
                                            config.registry_coordinator_addr();
                                            let operator_statr_retriever_addr_result =
                                            config.operator_state_retriever_addr();
                                            
                                            match registry_coordinator_addr_result {
                                                Ok(registry_coordinator_addr) => {
                                                    match operator_statr_retriever_addr_result {
                                                        Ok(operator_statr_retriever_addr) => {
                                                            let operator_address_result =
                                                            config.operator_address();
                                                            
                                                            match operator_address_result{
                                                                
                                                                Ok(operator_address) =>{
                                                                    Ok(Self {
                                                                        rpc_url: config.get_rpc_url(),
                                                                        operator_addr: operator_address,
                                                                        key_pair: BlsKeypair::default(),
                                                                        operator_id: operator_id,
                                                                        client: ClientAggregator::new(
                                                                            config.aggregator_ip_addr(),
                                                                        ),
                                                                        metrics,
                                                                        aggregator_ip_addr: config
                                                                            .aggregator_ip_addr(),
                                                                        signer,
                                                                        registry_coordinator:
                                                                            registry_coordinator_addr,
                                                                        operator_state_retriever:
                                                                            operator_statr_retriever_addr,
                                                                    })
                                                                }
                                                                Err(e) =>{
                                                                    return Err(OperatorError::ConfigParseError(e))
                                                                }

                                                            }
                                                        }

                                                        Err(e) => {
                                                            return Err(
                                                                OperatorError::ConfigParseError(e),
                                                            )
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    return Err(OperatorError::ConfigParseError(e));
                                                }
                                            }
                                        }
                                        Err(e) => Err(OperatorError::ConfigParseError(e)),
                                    }
                                } else {
                                    Err(OperatorError::EncodedKeystore)
                                }
                            }
                        }
                    }
                    Err(_) => Err(OperatorError::BlsKeystorePath),
                }
            }
            Err(e) => {
                debug!("Error is {e:?}");
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
        let avs_registry_reader_result = AvsRegistryChainReader::new(
            self.registry_coordinator,
            self.operator_state_retriever,
            self.rpc_url.clone(),
        )
        .await;

        match avs_registry_reader_result {
            Ok(avs_registry_reader) => {
                let is_registered_result = avs_registry_reader
                    .is_operator_registered(self.operator_addr.clone())
                    .await;

                if is_registered_result.is_ok() {
                    info!("Starting operator");

                    let ws = WsConnect::new(self.rpc_url.clone());
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
                    }
                }

                Ok(())
            }
            Err(_) => Err(AvsRegistryError::BuildElChainReader.into()),
        }
    }

    /// Sign the task response
    pub fn sign_task_response(
        &self,
        task_response: TaskResponse,
    ) -> Result<SignedTaskResponse, OperatorError> {
        let encoded_response = TaskResponse::abi_encode(&task_response);
        debug!("encoded response: {:?}", encoded_response);
        let hash_msg = keccak256(encoded_response);

        let signed_msg_result = sign(self.key_pair.priv_key(), hash_msg.as_slice());

        match signed_msg_result {
            Ok(signature) => {
                debug!("signature : {:?}", signature);
                let signed_task_response =
                    SignedTaskResponse::new(task_response, signature, self.operator_id);
                info!("signed task response : {:?}", signed_task_response);
                Ok(signed_task_response)
            }
            Err(_) => Err(OperatorError::SignUsingBlsKeyPair),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_new_task() {}

    #[test]
    fn test_build_operator() {}

    #[test]
    fn test_sign_task_response() {}
}
