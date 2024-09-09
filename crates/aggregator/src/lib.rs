//! Aggregator crate

/// Aggregator error
pub mod error;
pub mod fake_aggregator;
/// RPC server
pub mod rpc_server;
use alloy::dyn_abi::SolType;
use alloy::providers::Provider;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::rpc::types::Filter;
use alloy::sol_types::SolEvent;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use eigen_logging::get_logger;
use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigen_services_blsaggregation::bls_agg::{
    BlsAggregationServiceError, BlsAggregationServiceResponse, BlsAggregatorService,
};
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigen_types::avs::TaskResponseDigest;
use eigen_types::operator::Socket;
use eigen_utils::get_ws_provider;
pub use error::AggregatorError;
use futures_util::StreamExt;
use incredible_bindings::IncredibleSquaringTaskManager::NewTaskCreated;
use incredible_bindings::IncredibleSquaringTaskManager::{self, NonSignerStakesAndSignature};
use incredible_chainio::AvsWriter;
use incredible_config::IncredibleConfig;
use jsonrpc_core::serde_json;
use jsonrpc_core::{Error, IoHandler, Params, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
pub use rpc_server::SignedTaskResponse;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

/// Task Challenge Window Block : 100 blocks
pub const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
/// Block Time Seconds : 12 seconds
pub const BLOCK_TIME_SECONDS: u32 = 12;

/// Aggregator
#[derive(Debug)]
pub struct Aggregator {
    port_address: String,
    avs_writer: AvsWriter,
    bls_aggregation_service: BlsAggregatorService<
        AvsRegistryServiceChainCaller<AvsRegistryChainReader, OperatorInfoServiceInMemory>,
    >,
    /// HashMap to store tasks
    pub tasks: HashMap<u32, IncredibleSquaringTaskManager::Task>,
    /// HashMap to store task responses
    pub tasks_responses:
        HashMap<u32, HashMap<TaskResponseDigest, IncredibleSquaringTaskManager::TaskResponse>>,
}

impl Aggregator {
    /// Creates a new aggregator
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the aggregator
    ///
    /// # Returns
    ///
    /// * `Self` - The aggregator
    pub async fn new(config: IncredibleConfig) -> Result<Self, AggregatorError> {
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            get_logger(),
            config.registry_coordinator_addr()?,
            config.operator_state_retriever_addr()?,
            config.http_rpc_url(),
        )
        .await?;

        let avs_writer = AvsWriter::new(
            config.registry_coordinator_addr()?,
            config.http_rpc_url(),
            config.get_signer(),
        )
        .await?;

        let operators_info_service = OperatorInfoServiceInMemory::new(
            get_logger(),
            avs_registry_chain_reader.clone(),
            config.ws_rpc_url(),
        )
        .await;
        let token = tokio_util::sync::CancellationToken::new().clone();
        let avs_registry_service_chaincaller = AvsRegistryServiceChainCaller::new(
            avs_registry_chain_reader,
            operators_info_service.clone(),
        );
        let provider = get_ws_provider(config.ws_rpc_url().as_str()).await?;

        let current_block_number = provider.get_block_number().await?;
        tokio::spawn(async move {
            let _ = operators_info_service
                .start_service(&token, 0, current_block_number)
                .await;
        });

        let bls_aggregation_service = BlsAggregatorService::new(avs_registry_service_chaincaller);

        Ok(Self {
            port_address: config.aggregator_ip_addr(),
            avs_writer,
            tasks_responses: HashMap::new(),
            tasks: HashMap::new(),
            bls_aggregation_service,
        })
    }

    /// Starts the aggregator service
    pub async fn start(self, ws_rpc_url: String) -> eyre::Result<()> {
        info!("Starting aggregator");

        let aggregator = Arc::new(tokio::sync::Mutex::new(self));

        // Spawn two tasks: one for the server and one for processing tasks
        let server_handle = tokio::spawn(Self::start_server(Arc::clone(&aggregator)));
        let process_handle = tokio::spawn(Self::process_tasks(
            ws_rpc_url.clone(),
            Arc::clone(&aggregator),
        ));

        // Wait for both tasks to complete and handle potential errors
        match tokio::try_join!(server_handle, process_handle) {
            Ok((server_result, process_result)) => {
                server_result?;
                process_result?;
            }
            Err(e) => {
                eprintln!("Error in task execution: {:?}", e);
                return Err(eyre::eyre!("Task execution failed"));
            }
        }

        Ok(())
    }

    /// Starts the RPC server
    ///
    /// # Arguments
    ///
    /// * `aggregator` - The aggregator
    ///
    /// # Returns
    ///
    /// * `eyre::Result<()>` - The result of the operation
    pub async fn start_server(
        aggregator: Arc<tokio::sync::Mutex<Self>>,
    ) -> eyre::Result<(), AggregatorError> {
        let mut io = IoHandler::new();
        io.add_method("process_signed_task_response", {
            let aggregator = Arc::clone(&aggregator);
            move |params: Params| {
                let aggregator = Arc::clone(&aggregator);
                async move {
                    let signed_task_response: SignedTaskResponse = match params {
                        Params::Map(map) => serde_json::from_value(map["params"].clone()).expect(
                            "Error in adding method in io handler for start_server function",
                        ),
                        _ => return { Err(Error::invalid_params("Expected a map")) },
                    };

                    // Call the process_signed_task_response function
                    let result = aggregator
                        .lock()
                        .await
                        .process_signed_task_response(signed_task_response)
                        .await;
                    match result {
                        Ok(_) => Ok(Value::Bool(true)),
                        Err(_) => Err(Error::invalid_params("invalid")),
                    }
                }
            }
        });
        let socket: SocketAddr = aggregator.lock().await.port_address.parse().map_err(|e| {
            AggregatorError::IOError(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        })?;
        let server = ServerBuilder::new(io)
            .cors(DomainsValidation::AllowOnly(vec![
                AccessControlAllowOrigin::Any,
            ]))
            .start_http(&socket)?;

        info!("Server running at {}", socket);

        server.wait();

        Ok(())
    }

    /// Processes the tasks
    ///
    /// # Arguments
    ///
    /// * `ws_rpc_url` - The websocket RPC URL
    /// * `aggregator` - The aggregator
    ///
    /// # Returns
    ///
    /// * `eyre::Result<()>` - The result of the operation
    pub async fn process_tasks(
        ws_rpc_url: String,
        aggregator: Arc<tokio::sync::Mutex<Self>>,
    ) -> eyre::Result<()> {
        let ws = WsConnect::new(ws_rpc_url.clone());
        let provider = ProviderBuilder::new().on_ws(ws).await?;

        let filter = Filter::new().event_signature(NewTaskCreated::SIGNATURE_HASH);
        let sub = provider.subscribe_logs(&filter).await?;
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            info!("received new task created in aggregator ");

            let NewTaskCreated { taskIndex, task } = log.log_decode()?.inner.data;

            aggregator
                .lock()
                .await
                .tasks
                .insert(taskIndex, task.clone());

            let mut quorum_nums: Vec<u8> = vec![];
            let mut quorum_threshold_percentages = Vec::with_capacity(task.quorumNumbers.len());
            for _ in &task.quorumNumbers {
                quorum_threshold_percentages.push(task.quorumThresholdPercentage.try_into()?);
            }

            for (_, val) in task.quorumNumbers.iter().enumerate() {
                quorum_nums.push(*val);
            }

            let time_to_expiry = tokio::time::Duration::from_secs(
                (TASK_CHALLENGE_WINDOW_BLOCK * BLOCK_TIME_SECONDS).into(),
            );
            info!("initializing new task in bls aggregation service");
            let _ = aggregator
                .lock()
                .await
                .bls_aggregation_service
                .initialize_new_task(
                    taskIndex,
                    task.taskCreatedBlock,
                    quorum_nums.clone(),
                    quorum_threshold_percentages.clone(),
                    time_to_expiry,
                )
                .await
                .map_err(|e: BlsAggregationServiceError| eyre::eyre!(e));

            info!("initialized new task in bls aggregation service");
        }

        Ok(())
    }

    /// Processes the signed task response
    ///
    /// # Arguments
    ///
    /// * [`SignedTaskResponse`] - The signed task response
    ///
    /// # Returns
    ///
    /// * `eyre::Result<()>` - The result of the operation
    pub async fn process_signed_task_response(
        &mut self,
        signed_task_response: SignedTaskResponse,
    ) -> Result<(), AggregatorError> {
        let task_index = signed_task_response.task_response.referenceTaskIndex;

        let task_response_digest =
            alloy::primitives::keccak256(IncredibleSquaringTaskManager::TaskResponse::abi_encode(
                &signed_task_response.task_response,
            ));

        let response =
            check_double_mapping(&self.tasks_responses, task_index, task_response_digest);

        if response.is_none() {
            let mut inner_map = HashMap::new();
            inner_map.insert(
                task_response_digest,
                signed_task_response.clone().task_response,
            );
            self.tasks_responses.insert(task_index, inner_map);
        }

        self.bls_aggregation_service
            .process_new_signature(
                task_index,
                task_response_digest,
                signed_task_response.signature(),
                signed_task_response.operator_id(),
            )
            .await?;
        info!("processed signature for index {:?}", task_index);

        if let Some(aggregated_response) = self
            .bls_aggregation_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await
        {
            info!("sending aggregated response to contract");
            self.send_aggregated_response_to_contract(aggregated_response?)
                .await?;
        }
        Ok(())
    }

    /// Sends the aggregated response to the contract
    ///
    /// # Arguments
    ///
    /// * [`BlsAggregationServiceResponse`] - The aggregated response
    ///
    /// # Returns
    ///
    /// * `eyre::Result<()>` - The result of the operation
    pub async fn send_aggregated_response_to_contract(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), AggregatorError> {
        let mut non_signer_pub_keys = Vec::<IncredibleSquaringTaskManager::G1Point>::new();
        for pub_key in response.non_signers_pub_keys_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1())?;
            non_signer_pub_keys.push(IncredibleSquaringTaskManager::G1Point { X: g1.X, Y: g1.Y })
        }

        let mut quorum_apks = Vec::<IncredibleSquaringTaskManager::G1Point>::new();
        for pub_key in response.quorum_apks_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1())?;
            quorum_apks.push(IncredibleSquaringTaskManager::G1Point { X: g1.X, Y: g1.Y })
        }

        let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
            nonSignerPubkeys: non_signer_pub_keys,
            nonSignerQuorumBitmapIndices: response.non_signer_quorum_bitmap_indices,
            quorumApks: quorum_apks,
            apkG2: IncredibleSquaringTaskManager::G2Point {
                X: convert_to_g2_point(response.signers_apk_g2.g2())?.X,
                Y: convert_to_g2_point(response.signers_apk_g2.g2())?.Y,
            },
            sigma: IncredibleSquaringTaskManager::G1Point {
                X: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())?.X,
                Y: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())?.Y,
            },
            quorumApkIndices: response.quorum_apk_indices,
            totalStakeIndices: response.total_stake_indices,
            nonSignerStakeIndices: response.non_signer_stake_indices,
        };

        let task = &self.tasks[&response.task_index];
        let task_response =
            &self.tasks_responses[&response.task_index][&response.task_response_digest];
        self.avs_writer
            .send_aggregated_response(
                task.clone(),
                task_response.clone(),
                non_signer_stakes_and_signature,
            )
            .await?;
        Ok(())
    }
}

fn check_double_mapping(
    outer_map: &HashMap<
        u32,
        HashMap<TaskResponseDigest, IncredibleSquaringTaskManager::TaskResponse>,
    >,
    outer_key: u32,
    inner_key: TaskResponseDigest,
) -> Option<&IncredibleSquaringTaskManager::TaskResponse> {
    if let Some(inner_map) = outer_map.get(&outer_key) {
        if let Some(value) = inner_map.get(&inner_key) {
            return Some(value);
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_double_mapping() {
        let mut outer_map = HashMap::new();
        let mut inner_map = HashMap::new();
        inner_map.insert(
            TaskResponseDigest::default(),
            IncredibleSquaringTaskManager::TaskResponse {
                referenceTaskIndex: "0".parse().unwrap(),
                numberSquared: "0".parse().unwrap(),
            },
        );
        outer_map.insert(1, inner_map);
        let result = check_double_mapping(&outer_map, 1, TaskResponseDigest::default());
        assert!(result.is_some());
    }
}
