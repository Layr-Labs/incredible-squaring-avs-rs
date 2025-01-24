//! Aggregator crate

/// Aggregator error
pub mod error;
#[allow(missing_docs)]
pub mod fake_aggregator;
/// RPC server
pub mod rpc_server;

mod aggregator_impl;
pub use aggregator_impl::ISTaskProcessor;

use alloy::primitives::B256;
use alloy::providers::Provider;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::rpc::types::Filter;
use alloy::sol_types::SolEvent;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_common::get_ws_provider;
use eigen_logging::get_logger;
use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigen_services_blsaggregation::bls_agg::BlsAggregatorService;
use eigen_services_blsaggregation::bls_aggregation_service_error::BlsAggregationServiceError;
use eigen_services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigen_types::{avs::TaskIndex, operator::QuorumThresholdPercentages};
use futures_util::StreamExt;
use incredible_config::IncredibleConfig;
use jsonrpc_core::serde_json;
use jsonrpc_core::{Error, IoHandler, Params, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use rpc_server::SignedTaskResponseImpl;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, future::Future, net::SocketAddr, sync::Arc, time::Duration};
use tracing::info;

pub use error::AggregatorError;
pub use rpc_server::SignedTaskResponse;

// TODO: maybe `TaskMetadata` would be a better name
#[derive(Debug)]
/// Metadata related to a task. Used for signature aggregation.
pub struct TaskInfo {
    /// Index of the task
    pub task_index: TaskIndex,
    /// Block the task was created
    pub task_created_block: u32,
    /// Quorum numbers which should respond to the task
    pub quorum_nums: Vec<u8>,
    /// Thresholds for each quorum
    pub quorum_threshold_percentages: QuorumThresholdPercentages,
    /// Time before expiry of the task response aggregation
    pub time_to_expiry: Duration,
}

/// Abstracts task-specific behaviour
pub trait TaskProcessor {
    /// The event type expected by the task processor
    type NewTaskEvent: SolEvent + Send + Sync + 'static;

    /// The response type expected by the task processor
    type TaskResponse: TaskResponse + Send + Sync + 'static;

    /// Processes a task, returning metadata related to signature aggregation
    fn process_new_task(&self, event: Self::NewTaskEvent) -> impl Future<Output = TaskInfo> + Send;

    /// Processes a task response, returning the response's digest
    fn process_task_response(&self, event: Self::TaskResponse)
        -> impl Future<Output = B256> + Send;

    /// Process the result of a BLS aggregation
    fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> impl Future<Output = ()> + Send;
}

/// Task response trait
pub trait TaskResponse: for<'de> Deserialize<'de> + Serialize {
    /// Returns the index of the task
    fn task_index(&self) -> TaskIndex;
}

/// Aggregator
#[derive(Debug)]
pub struct Aggregator<TP> {
    port_address: String,
    bls_aggregation_service: BlsAggregatorService<
        AvsRegistryServiceChainCaller<AvsRegistryChainReader, OperatorInfoServiceInMemory>,
    >,
    task_quorum: HashMap<u32, u32>,

    tp: TP,
}

impl<TP: TaskProcessor + Send + 'static> Aggregator<TP> {
    /// Creates a new aggregator
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the aggregator
    ///
    /// # Returns
    ///
    /// * `Self` - The aggregator
    pub async fn new(config: IncredibleConfig, tp: TP) -> Result<Self, AggregatorError> {
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            get_logger(),
            config.registry_coordinator_addr()?,
            config.operator_state_retriever_addr()?,
            config.http_rpc_url(),
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

        // let tp = ISTaskProcessor::new(
        //     config.registry_coordinator_addr()?,
        //     config.http_rpc_url(),
        //     config.get_signer(),
        // )
        // .await;

        Ok(Self {
            port_address: config.aggregator_ip_addr(),
            task_quorum: HashMap::new(),
            bls_aggregation_service,
            tp,
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
                    let Params::Map(map) = params else {
                        return Err(Error::invalid_params("Expected a map"));
                    };
                    let signed_task_response: SignedTaskResponseImpl<TP::TaskResponse> =
                        serde_json::from_value(map["params"].clone()).expect(
                            "Error in adding method in io handler for start_server function",
                        );
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

        let filter = Filter::new().event_signature(TP::NewTaskEvent::SIGNATURE_HASH);
        let sub = provider.subscribe_logs(&filter).await?;
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            let event: TP::NewTaskEvent = log.log_decode()?.inner.data;

            let info = aggregator.lock().await.tp.process_new_task(event).await;

            let _ = aggregator
                .lock()
                .await
                .bls_aggregation_service
                .initialize_new_task(
                    info.task_index,
                    info.task_created_block,
                    info.quorum_nums,
                    info.quorum_threshold_percentages,
                    info.time_to_expiry,
                )
                .await
                .map_err(|e: BlsAggregationServiceError| eyre::eyre!(e));
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
        signed_task_response: SignedTaskResponseImpl<TP::TaskResponse>,
    ) -> Result<(), AggregatorError> {
        let SignedTaskResponseImpl {
            task_response,
            signature,
            operator_id,
        } = signed_task_response;
        let task_index = task_response.task_index();

        let task_response_digest = self.tp.process_task_response(task_response).await;

        self.bls_aggregation_service
            .process_new_signature(task_index, task_response_digest, signature, operator_id)
            .await?;
        info!("processed signature for index {:?}", task_index);
        let quorum_reached = {
            let entry = self.task_quorum.entry(task_index).or_insert(0);
            *entry += 1;
            *entry >= 2
        };

        if quorum_reached {
            info!("quorum reached for task index: {:?}", task_index);
            if let Some(aggregated_response) = self
                .bls_aggregation_service
                .aggregated_response_receiver
                .lock()
                .await
                .recv()
                .await
            {
                info!("sending aggregated response to contract");
                // TODO: add error handling
                self.tp
                    .process_aggregated_response(aggregated_response?)
                    .await;
            }
        } else {
            info!(
                "quorum not reached yet for index:{:?}. waiting to receive more signatures ",
                task_index
            );
        }
        Ok(())
    }
}
