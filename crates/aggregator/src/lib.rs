//! Aggregator crate

/// Aggregator error
pub mod error;
/// RPC server
pub mod rpc_server;
use alloy::dyn_abi::SolType;
use alloy::providers::Provider;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::rpc::types::Filter;
use alloy::sol_types::SolEvent;
use ark_ec::AffineRepr;
use eigensdk::client_avsregistry::reader::AvsRegistryChainReader;
use eigensdk::common::get_ws_provider;
use eigensdk::crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use eigensdk::logging::get_logger;
use eigensdk::services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigensdk::services_blsaggregation::bls_agg::{
    AggregateReceiver, BlsAggregatorService, ServiceHandle, TaskMetadata, TaskSignature,
};
use eigensdk::services_blsaggregation::bls_aggregation_service_error::BlsAggregationServiceError;
use eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigensdk::services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigensdk::types::avs::TaskResponseDigest;
pub use error::AggregatorError;
use futures_util::StreamExt;
use incredible_bindings::incrediblesquaringtaskmanager::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    Task, TaskResponse,
};
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use incredible_bindings::incrediblesquaringtaskmanager::BN254::{G1Point, G2Point};
use incredible_chainio::AvsWriter;
use incredible_config::IncredibleConfig;
use jsonrpsee::server::{RpcModule, Server};
use jsonrpsee::types::ErrorObject;
pub use rpc_server::SignedTaskResponse;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::info;

/// Task Challenge Window Block : 100 blocks
pub const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
/// Block Time Seconds : 12 seconds
pub const BLOCK_TIME_SECONDS: u32 = 12;

/// Aggregator
#[derive(Debug)]
pub struct Aggregator {
    /// Socket address
    port_address: String,
    /// AVS writer
    avs_writer: AvsWriter,
    /// HashMap to store tasks
    tasks: HashMap<u32, Task>,
    /// HashMap to store tasks responses
    tasks_responses: HashMap<u32, HashMap<TaskResponseDigest, TaskResponse>>,
    /// Service handle to interact with the BLS Aggregator Service
    service_handle: ServiceHandle,
    /// To receive aggregated responses from the BLS Aggregator Service
    aggregate_receiver: AggregateReceiver,
}

impl Aggregator {
    /// Creates a new aggregator
    ///
    /// # Arguments
    ///
    /// * [`IncredibleConfig`] - The configuration for the aggregator
    ///
    /// # Returns
    ///
    /// * `Self` - The aggregator
    ///
    /// # Errors
    ///
    /// * `AggregatorError` - The error that occurred
    pub async fn new(config: IncredibleConfig) -> Result<Self, AggregatorError> {
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            get_logger(),
            config.registry_coordinator_addr()?,
            config.operator_state_retriever_addr()?,
            config.http_rpc_url(),
        )
        .await?;

        let avs_writer = AvsWriter::new(
            config.service_manager_addr()?,
            config.http_rpc_url(),
            config.get_signer(),
        )
        .await?;

        let operators_info_service = OperatorInfoServiceInMemory::new(
            get_logger(),
            avs_registry_chain_reader.clone(),
            config.ws_rpc_url(),
        )
        .await?
        .0;
        let token = tokio_util::sync::CancellationToken::new();
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

        let bls_aggregation_service =
            BlsAggregatorService::new(avs_registry_service_chaincaller, get_logger());

        let (handle, aggregate_receiver) = bls_aggregation_service.start();
        Ok(Self {
            port_address: config.aggregator_ip_addr(),
            avs_writer,
            tasks_responses: HashMap::new(),
            tasks: HashMap::new(),
            service_handle: handle,
            aggregate_receiver,
        })
    }

    /// Starts the aggregator service with three tasks: one for the server,
    /// one for processing tasks and one for processing aggregator responses.
    ///
    /// # Arguments
    ///
    /// * `ws_rpc_url` - The websocket RPC URL
    ///
    /// # Returns
    ///
    /// * `eyre::Result<()>` - The result of the operation
    ///
    /// # Errors
    ///
    /// * The error that occurred
    pub async fn start(self, ws_rpc_url: String) -> eyre::Result<()> {
        info!("Starting aggregator");

        let Self {
            avs_writer,
            port_address,
            tasks,
            tasks_responses,
            service_handle,
            aggregate_receiver,
        } = self;

        let task_responses = Arc::new(tokio::sync::Mutex::new(tasks_responses));
        let tasks = Arc::new(tokio::sync::Mutex::new(tasks));

        // Spawn three tasks: one for the server, one for processing tasks and one for processing aggregator responses
        // 1) Process signatures
        let server_handle =
            Self::start_server(port_address, service_handle.clone(), task_responses.clone())
                .await?;
        // 2) Process tasks
        let process_handle = tokio::spawn(Self::process_tasks(
            ws_rpc_url.clone(),
            Arc::clone(&tasks),
            service_handle,
        ));
        // 3) Process aggregator responses
        let responses_handle = tokio::spawn(Self::process_aggregator_responses(
            Arc::clone(&tasks),
            Arc::clone(&task_responses),
            avs_writer,
            aggregate_receiver,
        ));

        // Wait for the tasks to complete and handle potential errors
        let (_server_result, process_result, responses_result) =
            tokio::try_join!(server_handle, process_handle, responses_handle)
                .inspect_err(|e| eprintln!("Error in task execution: {e:?}"))
                .map_err(|e| eyre::eyre!("Task execution failed {e}"))?;

        process_result?;
        responses_result?;

        Ok(())
    }

    /// Starts the RPC server and returns a future that ends once the server is stopped
    async fn start_server(
        port_address: String,
        service_handle: ServiceHandle,
        task_responses: Arc<
            tokio::sync::Mutex<HashMap<u32, HashMap<TaskResponseDigest, TaskResponse>>>,
        >,
    ) -> eyre::Result<JoinHandle<()>, AggregatorError> {
        // See https://github.com/paritytech/jsonrpsee/blob/42461391fee47c94d42c4a7303355525291df9f6/examples/examples/cors_server.rs
        let mut module = RpcModule::new((service_handle, task_responses));
        module
            .register_async_method("process_signed_task_response", async |params, ctx, _| {
                let (service_handle, task_responses) = ctx.as_ref();
                let signed_task_response: SignedTaskResponse = params
                    .one()
                    .map_err(|err| ErrorObject::owned(0, err.to_string(), None::<()>))?;

                // Call the process_signed_task_response function
                let mut task_responses_lock = task_responses.lock().await;
                let result = Self::process_signed_task_response(
                    signed_task_response,
                    service_handle,
                    &mut task_responses_lock,
                )
                .await;

                match result {
                    Ok(()) => Ok(true),
                    Err(err) => Err(ErrorObject::owned(0, err.to_string(), None::<()>)),
                }
            })
            .expect("method name is unique");
        let socket: SocketAddr = port_address.parse().map_err(|e| {
            AggregatorError::IOError(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        })?;
        let middleware = tower::ServiceBuilder::new();
        // let cors = CorsLayer::new()
        //     // Allow `POST` when accessing the resource
        //     .allow_methods([Method::POST])
        //     // Allow requests from any origin
        //     .allow_origin(Any)
        //     .allow_headers([hyper::header::CONTENT_TYPE]);
        // let middleware = middleware.layer(cors);
        let server = Server::builder()
            .set_http_middleware(middleware)
            .build(socket)
            .await?;
        // let server = ServerBuilder::new(io)
        //     .cors(DomainsValidation::AllowOnly(vec![
        //         AccessControlAllowOrigin::Any,
        //     ]))
        //     .start_http(&socket)?;

        let handle = server.start(module);

        info!("Server running at {socket}");

        Ok(tokio::spawn(handle.stopped()))
    }

    /// Processes a signed task response received from an operator
    async fn process_signed_task_response(
        signed_task_response: SignedTaskResponse,
        service_handle: &ServiceHandle,
        task_responses: &mut HashMap<u32, HashMap<TaskResponseDigest, TaskResponse>>,
    ) -> Result<(), AggregatorError> {
        let task_index = signed_task_response.task_response.referenceTaskIndex;

        let task_response_digest = alloy::primitives::keccak256(TaskResponse::abi_encode(
            &signed_task_response.task_response,
        ));

        let signature = signed_task_response.signature();
        let operator_id = signed_task_response.operator_id();

        let task_signature =
            TaskSignature::new(task_index, task_response_digest, signature, operator_id);
        let result = service_handle.process_signature(task_signature).await;

        if result.is_err() {
            info!("Response received for task that was already completed");
            // TODO: Review if we need to return an error here
            return Ok(());
        }

        task_responses
            .entry(task_index)
            .or_default()
            .entry(task_response_digest)
            .or_insert(signed_task_response.task_response);

        Ok(())
    }

    /// Processes new tasks created in the task manager contract
    async fn process_tasks(
        ws_rpc_url: String,
        tasks: Arc<tokio::sync::Mutex<HashMap<u32, Task>>>,
        service_handle: ServiceHandle,
    ) -> eyre::Result<()> {
        let ws = WsConnect::new(ws_rpc_url.clone());
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .on_ws(ws)
            .await?;

        let filter = Filter::new().event_signature(NewTaskCreated::SIGNATURE_HASH);
        let sub = provider.subscribe_logs(&filter).await?;
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            let NewTaskCreated { taskIndex, task } = log.log_decode()?.inner.data;

            tasks.lock().await.insert(taskIndex, task.clone());

            let mut quorum_nums: Vec<u8> = vec![];
            let mut quorum_threshold_percentages = Vec::with_capacity(task.quorumNumbers.len());
            for _ in &task.quorumNumbers {
                quorum_threshold_percentages.push(task.quorumThresholdPercentage.try_into()?);
            }

            for val in task.quorumNumbers.iter() {
                quorum_nums.push(*val);
            }

            let time_to_expiry = tokio::time::Duration::from_secs(
                (TASK_CHALLENGE_WINDOW_BLOCK * BLOCK_TIME_SECONDS).into(),
            );
            let task_metadata = TaskMetadata::new(
                taskIndex,
                task.taskCreatedBlock.into(),
                quorum_nums.clone(),
                quorum_threshold_percentages.clone(),
                time_to_expiry,
            );

            let _ = service_handle
                .initialize_task(task_metadata)
                .await
                .map_err(|e: BlsAggregationServiceError| eyre::eyre!(e));
        }

        Ok(())
    }

    /// Processes BLS Aggregator Service responses
    async fn process_aggregator_responses(
        tasks: Arc<tokio::sync::Mutex<HashMap<u32, Task>>>,
        task_responses: Arc<
            tokio::sync::Mutex<HashMap<u32, HashMap<TaskResponseDigest, TaskResponse>>>,
        >,
        avs_writer: AvsWriter,
        mut aggregate_receiver_channel: AggregateReceiver,
    ) -> eyre::Result<()> {
        loop {
            // Wait for the next response of the aggregate receiver channel
            let Ok(service_response) = aggregate_receiver_channel
                .receive_aggregated_response()
                .await
                .inspect(|service_response| {
                    info!(
                        "Received aggregated response for task_index: {}",
                        service_response.task_index
                    )
                })
                .inspect_err(|e| info!("Error receiving aggregated response: {:?}", e))
            else {
                continue;
            };

            let task_response = task_responses
                .lock()
                .await
                .get(&service_response.task_index)
                .and_then(|map| map.get(&service_response.task_response_digest))
                .cloned();

            if let Some(task_response) = task_response {
                let tasks_lock = tasks.lock().await;
                send_aggregated_response_to_contract(
                    &tasks_lock,
                    &avs_writer,
                    task_response,
                    service_response,
                )
                .await?;
            } else {
                info!(
                    "Not found task_response for task_index: {}",
                    service_response.task_index
                );
            }
        }
    }
}

/// Sends an aggregated task response to the contract
async fn send_aggregated_response_to_contract(
    tasks: &HashMap<u32, Task>,
    avs_writer: &AvsWriter,
    task_response: TaskResponse,
    response: BlsAggregationServiceResponse,
) -> Result<(), AggregatorError> {
    let mut non_signer_pub_keys = Vec::<G1Point>::new();
    for pub_key in response.non_signers_pub_keys_g1.iter() {
        if pub_key.g1().x().is_some() {
            let g1 = convert_to_g1_point(pub_key.g1())?;
            non_signer_pub_keys.push(G1Point { X: g1.X, Y: g1.Y })
        } else {
            info!(
                "Zero non_signers for the task index :{:?}",
                response.task_index
            );
        }
    }

    let mut quorum_apks = Vec::<G1Point>::new();
    for pub_key in response.quorum_apks_g1.iter() {
        let g1 = convert_to_g1_point(pub_key.g1())?;
        quorum_apks.push(G1Point { X: g1.X, Y: g1.Y })
    }

    let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
        nonSignerPubkeys: non_signer_pub_keys,
        nonSignerQuorumBitmapIndices: response.non_signer_quorum_bitmap_indices,
        quorumApks: quorum_apks,
        apkG2: G2Point {
            X: convert_to_g2_point(response.signers_apk_g2.g2())?.X,
            Y: convert_to_g2_point(response.signers_apk_g2.g2())?.Y,
        },
        sigma: G1Point {
            X: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())?.X,
            Y: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())?.Y,
        },
        quorumApkIndices: response.quorum_apk_indices,
        totalStakeIndices: response.total_stake_indices,
        nonSignerStakeIndices: response.non_signer_stake_indices,
    };

    let task = &tasks[&response.task_index];
    avs_writer
        .send_aggregated_response(task.clone(), task_response, non_signer_stakes_and_signature)
        .await?;
    Ok(())
}
