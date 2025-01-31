//! Builder module for the AVS. Starts all the services for the AVS using futures simulatenously.
use crate::avs_builder::AvsBuilder;
use crate::traits::LaunchAvs;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_logging::get_logger;
use eigen_nodeapi::{create_server, NodeApi};
use eigen_operator::traits::Operator;
use futures::TryFutureExt;
use incredible_aggregator::ISTaskProcessor;
use incredible_aggregator::{Aggregator, AggregatorConfig};
use incredible_challenger::Challenger;
use incredible_operator::incredible_square_operator::IncredibleSquareOperator;
use incredible_task_generator::TaskManager;
use ntex::rt::System;
use tracing::info;

/// Default avs launcher
#[derive(Debug)]
pub struct DefaultAvsLauncher {}

impl DefaultAvsLauncher {
    /// new
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for DefaultAvsLauncher {
    fn default() -> Self {
        Self::new()
    }
}

impl LaunchAvs<AvsBuilder> for DefaultAvsLauncher {
    async fn launch_avs(self, avs: AvsBuilder) -> eyre::Result<()> {
        info!("launching crates: incredible-squaring-avs-rs");
        incredible_metrics::new();
        // start operator
        let operator_builder = IncredibleSquareOperator::new(avs.config.clone()).await?;
        let operator_builder2 = IncredibleSquareOperator::new(avs.config.clone()).await?;

        println!("@@@ operator_builder 1 key {:?}", operator_builder.key_pair);
        println!(
            "@@@ operator_builder 2 key {:?}",
            operator_builder2.key_pair
        );

        let mut challenge = Challenger::build(avs.config.clone()).await?;
        let registry_coordinator = operator_builder.registry_coordinator;
        let operator_state_retriever = operator_builder.operator_state_retriever;
        let http_rpc_url = avs.config.http_rpc_url().clone();
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_logger(),
            registry_coordinator,
            operator_state_retriever,
            http_rpc_url,
        )
        .await?;

        // Start Operator 1
        let key_pair = &operator_builder.key_pair;
        let operator_id = &operator_builder.operator_id;
        let operator_address = operator_builder.operator_addr;
        let operator_name = "operator1";
        let client_aggregator = &operator_builder.client;
        let ws_rpc_url = &operator_builder.ws_rpc_url;
        let operator_service = IncredibleSquareOperator::start_operator(
            &avs_registry_reader,
            key_pair,
            operator_id,
            operator_address,
            operator_name,
            client_aggregator,
            ws_rpc_url,
        )
        .map_err(|e| eyre::eyre!("Operator error: {:?}", e));

        // Start operator 2
        let key_pair = &operator_builder2.key_pair;
        let operator_id = &operator_builder2.operator_id;
        let operator_address = operator_builder2.operator_addr;
        let operator_name = "operator2";
        let client_aggregator = operator_builder.client.clone();
        let ws_rpc_url = &operator_builder2.ws_rpc_url;
        let operator2_service = IncredibleSquareOperator::start_operator(
            &avs_registry_reader,
            key_pair,
            operator_id,
            operator_address,
            operator_name,
            &client_aggregator,
            ws_rpc_url,
        )
        .map_err(|e| eyre::eyre!("Operator error: {:?}", e));

        let challenger_service = challenge
            .start_challenger()
            .map_err(|e| eyre::eyre!("Challenger error: {:?}", e));

        let task_processor = ISTaskProcessor::new(
            avs.config.registry_coordinator_addr()?,
            avs.config.http_rpc_url(),
            avs.config.get_signer(),
        )
        .await?;

        let aggregator_cfg = AggregatorConfig {
            server_address: avs.config.aggregator_ip_addr(),
            http_rpc_url: avs.config.http_rpc_url(),
            ws_rpc_url: avs.config.ws_rpc_url(),
            registry_coordinator: avs.config.registry_coordinator_addr()?,
            operator_state_retriever: avs.config.operator_state_retriever_addr()?,
        };
        let aggregator = Aggregator::new(aggregator_cfg, task_processor)
            .await
            .map_err(|e| eyre::eyre!("Aggregator new error {e:?}"))?;

        let aggregator_service_with_rpc_client = aggregator
            .start(avs.config.ws_rpc_url().clone())
            .map_err(|e| eyre::eyre!("Aggregator start error {e:?}"));

        let task_manager = TaskManager::new(
            avs.config.task_manager_addr()?,
            avs.config.http_rpc_url(),
            avs.config.task_manager_signer(),
        );

        let task_spam_service = task_manager
            .start()
            .map_err(|e| eyre::eyre!("Task manager error {e:?}"));

        let node_api = NodeApi::new("incredible-squaring", "v0.0.1");
        let node_api_address = avs.config.node_api_port_address();
        info!("node_api_address{:?}", node_api_address);

        std::thread::spawn(move || {
            let _ = System::new("node_api_system").block_on(async move {
                let node_api_server = create_server(node_api, node_api_address).unwrap();
                node_api_server.await
            });
        });

        let _ = futures::future::try_join5(
            operator_service,
            operator2_service,
            challenger_service,
            aggregator_service_with_rpc_client,
            task_spam_service,
        )
        .await?;

        Ok(())
    }
}
