//! Starts all the services for the AVS simultaneously.
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_logging::get_logger;
use eigen_nodeapi::{create_server, NodeApi};
use eigen_operator::traits::Operator;
use futures::TryFutureExt;
use incredible_aggregator::ISTaskProcessor;
use incredible_aggregator::{Aggregator, AggregatorConfig};
use incredible_challenger::Challenger;
use incredible_config::IncredibleConfig;
use incredible_operator::incredible_square_operator::IncredibleSquareOperator;
use incredible_task_generator::TaskManager;
use ntex::rt::System;
use tokio::task::JoinSet;
use tracing::info;

/// Launches all the services for the AVS.
pub async fn launch_avs(avs_config: IncredibleConfig) -> eyre::Result<()> {
    info!("launching crates: incredible-squaring-avs-rs");
    incredible_metrics::new();

    let registry_coordinator = avs_config.registry_coordinator_addr()?;
    let operator_state_retriever = avs_config.operator_state_retriever_addr()?;
    let http_rpc_url = avs_config.http_rpc_url().clone();
    let avs_registry_reader = AvsRegistryChainReader::new(
        get_logger(),
        registry_coordinator,
        operator_state_retriever,
        http_rpc_url,
    )
    .await?;

    // start operators
    let mut joinset = JoinSet::new();

    for (i, config) in avs_config.get_operator_configs().iter().enumerate() {
        let operator_builder =
            IncredibleSquareOperator::new(avs_config.clone(), config.clone()).await?;

        // println!(
        //     "@@@ operator_builder 1 key {:?}",
        //     operator_builders[0].key_pair
        // );
        let avs_registry_reader = avs_registry_reader.clone();

        joinset.spawn(async move {
            let key_pair = &operator_builder.key_pair;
            let operator_id = &operator_builder.operator_id;
            let operator_address = operator_builder.operator_addr;
            let operator_name = format!("operator{i}");
            let client_aggregator = &operator_builder.client;
            let ws_rpc_url = &operator_builder.ws_rpc_url;
            let operator_service = IncredibleSquareOperator::start_operator(
                &avs_registry_reader,
                key_pair,
                operator_id,
                operator_address,
                &operator_name,
                client_aggregator,
                ws_rpc_url,
            )
            .map_err(|e| eyre::eyre!("Operator error: {:?}", e));
            operator_service.await
        });
    }
    let mut challenge = Challenger::build(avs_config.clone()).await?;

    // Start Operator 1

    let challenger_service = challenge
        .start_challenger()
        .map_err(|e| eyre::eyre!("Challenger error: {:?}", e));

    let task_processor = ISTaskProcessor::new(
        avs_config.registry_coordinator_addr()?,
        avs_config.http_rpc_url(),
        avs_config.get_signer(),
    )
    .await?;

    let aggregator_cfg = AggregatorConfig {
        server_address: avs_config.aggregator_ip_addr(),
        http_rpc_url: avs_config.http_rpc_url(),
        ws_rpc_url: avs_config.ws_rpc_url(),
        registry_coordinator: avs_config.registry_coordinator_addr()?,
        operator_state_retriever: avs_config.operator_state_retriever_addr()?,
    };
    let aggregator = Aggregator::new(aggregator_cfg, task_processor)
        .await
        .map_err(|e| eyre::eyre!("Aggregator new error {e:?}"))?;

    let aggregator_service_with_rpc_client = aggregator
        .start(avs_config.ws_rpc_url().clone())
        .map_err(|e| eyre::eyre!("Aggregator start error {e:?}"));

    let task_manager = TaskManager::new(
        avs_config.task_manager_addr()?,
        avs_config.http_rpc_url(),
        avs_config.task_manager_signer(),
    );

    let task_spam_service = task_manager
        .start()
        .map_err(|e| eyre::eyre!("Task manager error {e:?}"));

    let node_api = NodeApi::new("incredible-squaring", "v0.0.1");
    let node_api_address = avs_config.node_api_port_address();
    info!("node_api_address{:?}", node_api_address);

    std::thread::spawn(move || {
        let _ = System::new("node_api_system").block_on(async move {
            let node_api_server = create_server(node_api, node_api_address).unwrap();
            node_api_server.await
        });
    });

    let _ = futures::future::try_join4(
        async move { Ok(joinset.join_all().await) },
        challenger_service,
        aggregator_service_with_rpc_client,
        task_spam_service,
    )
    .await?;

    Ok(())
}
