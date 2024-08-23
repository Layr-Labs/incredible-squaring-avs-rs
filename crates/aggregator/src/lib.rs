//! Aggregator crate
pub mod rpc_server;
use std::collections::HashMap;

use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_logging::get_logger;
use eigen_logging::logger::SharedLogger;
use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigen_services_blsaggregation::bls_agg::BlsAggregatorService;
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigen_types::avs::TaskResponseDigest;
use incredible_bindings::IncredibleSquaringTaskManager;
use incredible_bindings::IncredibleSquaringTaskManager::{
    respondToTaskCall, G1Point, NewTaskCreated, Task, TaskResponded, TaskResponseMetadata,
};
use incredible_chainio::AvsWriter;
use incredible_config::IncredibleConfig;
use jsonrpc_core::{Error, IoHandler, Params, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use rpc_server::{handle_signed_task_response, SignedTaskResponse};
use serde::Serialize;
use serde::Serializer;
use tracing::info;

///
#[derive(Debug)]
pub struct Aggregator<A: eigen_services_avsregistry::AvsRegistryService + Clone> {
    logger: SharedLogger,
    port_address: String,

    avs_writer: AvsWriter,

    bls_aggregation_service: BlsAggregatorService<A>,

    // bls_aggregation_service:,
    tasks: HashMap<u32, IncredibleSquaringTaskManager::Task>,

    tasks_responses:
        HashMap<u32, HashMap<TaskResponseDigest, IncredibleSquaringTaskManager::TaskResponse>>,
}

impl Aggregator<AvsRegistryServiceChainCaller> {
    pub async fn new(config: IncredibleConfig) -> Self {
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            get_logger(),
            config.registry_coordinator_addr().unwrap(),
            config.operator_state_retriever_addr().unwrap(),
            config.http_rpc_url(),
        )
        .await
        .unwrap();

        let avs_writer = AvsWriter::new(
            config.registry_coordinator_addr().unwrap(),
            config.http_rpc_url(),
            config.get_signer(),
        )
        .await
        .unwrap();

        let operators_info_service = OperatorInfoServiceInMemory::new(
            get_logger(),
            avs_registry_chain_reader,
            config.ws_rpc_url(),
        )
        .await;

        let avs_registry_service_chaincaller =
            AvsRegistryServiceChainCaller::new(operators_info_service);

        let bls_aggregation_service = BlsAggregatorService::new(avs_registry_service_chaincaller);

        Self {
            logger: get_logger(),
            port_address: config.aggregator_ip_addr(),
            avs_writer,
            tasks_responses: HashMap::new(),
            tasks: HashMap::new(),
            bls_aggregation_service,
        }
    }

    /// Starts the aggregator service
    pub async fn start(&self) -> eyre::Result<()> {
        info!("Starting aggregator");
        let mut io = IoHandler::new();

        io.add_method("process_signed_task_response", move |params: Params| {
            async move {
                let signed_task_response: SignedTaskResponse = params.parse()?;

                // Call the handle_signed_task_response function
                match handle_signed_task_response(signed_task_response) {
                    Ok(res) => Ok(Value::Bool(res)),
                    Err(err) => Err(Error::invalid_params(err)),
                }
            }
        });

        Ok(())
    }
}
