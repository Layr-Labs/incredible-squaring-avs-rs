//! Builder module for the AVS. Starts all the services for the AVS using futures simulatenously.
use alloy::{
    network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use eigensdk::{
    aggregator::{Aggregator, AggregatorConfig},
    nodeapi::{NodeApi, NodeInfo},
};
use futures::TryFutureExt;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager;
use incredible_config::IncredibleConfig;
use std::{future::Future, str::FromStr, sync::Arc};
use tracing::info;
/// Launch Avs trait
pub trait LaunchAvs<T: Send + 'static> {
    /// Launch Avs
    fn launch_avs(self, avs: T) -> impl Future<Output = eyre::Result<()>> + Send;
}

/// Avs builder
#[derive(Debug)]
pub struct AvsBuilder {
    config: IncredibleConfig,
}

impl AvsBuilder {
    /// new
    pub fn new(config: IncredibleConfig) -> Self {
        Self { config }
    }
}

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
        // info!("launching crates: incredible-squaring-avs-rs");
        // incredible_metrics::new();
        // // start operator
        // let mut operator_builder = OperatorBuilder::build(avs.config.clone()).await?;
        // let mut operator_builder2 = OperatorBuilder2::build(
        //     avs.config.clone(),
        //     Some(Arc::new(operator_builder.client.clone())),
        // )
        // .await?;

        // let mut challenge = Challenger::build(avs.config.clone()).await?;
        // let operator_service = operator_builder
        //     .start_operator()
        //     .map_err(|e| eyre::eyre!("Operator error: {:?}", e));

        // let operator2_service = operator_builder2
        //     .start_operator()
        //     .map_err(|e| eyre::eyre!("Operator error: {:?}", e));

        // let challenger_service = challenge
        //     .start_challenger()
        //     .map_err(|e| eyre::eyre!("Challenger error: {:?}", e));
        // let aggregator = Aggregator::new(avs.config.clone()).await?;
        // let aggregator_service_with_rpc_client = aggregator
        //     .start(avs.config.ws_rpc_url().clone())
        //     .map_err(|e| eyre::eyre!("Aggregator error {e:?}"));

        // let task_manager = TaskManager::new(
        //     avs.config.task_manager_addr()?,
        //     avs.config.http_rpc_url(),
        //     avs.config.task_manager_signer(),
        //     avs.config.quorum_number()?.to_string(),
        // );
        // let task_spam_service = task_manager
        //     .start()
        //     .map_err(|e| eyre::eyre!("Task manager error {e:?}"));
        // let node_info = NodeInfo::new("incredible-squaring", "v0.0.1");
        // let node_api = NodeApi::new(node_info);
        // let node_api_address = avs.config.node_api_port_address();
        // info!("node_api_address{:?}", node_api_address);

        // std::thread::spawn(move || {
        //     let _ = System::new("node_api_system").block_on(async move {
        //         let node_api_server = node_api.start_server(node_api_address.as_str()).unwrap();
        //         node_api_server.await
        //     });
        // });

        // let _ = futures::future::try_join5(
        //     operator_service,
        //     operator2_service,
        //     challenger_service,
        //     aggregator_service_with_rpc_client,
        //     task_spam_service,
        // )
        // .await?;

        // Create task manager contract instance
        let url = Url::parse(&avs.config.http_rpc_url())?;
        let signer = PrivateKeySigner::from_str(&avs.config.get_signer())?;
        let wallet = EthereumWallet::new(signer);
        let pr = ProviderBuilder::new().wallet(wallet).on_http(url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(avs.config.task_manager_addr()?, pr);

        // Launch aggregator
        let config_aggregator = AggregatorConfig {
            http_rpc_url: avs.config.http_rpc_url().to_string(),
            ws_rpc_url: avs.config.ws_rpc_url().to_string(),
            server_address: avs.config.aggregator_ip_addr().to_string(),
            registry_coordinator: avs.config.registry_coordinator_addr()?,
            operator_state_retriever: avs.config.operator_state_retriever_addr()?,
        };

        let aggregator = Aggregator::new(config_aggregator, task_manager_contract).await?;
        let aggregator_service_with_rpc_client = aggregator
            .start()
            .map_err(|e| eyre::eyre!("Aggregator error {e:?}"));

        Ok(())
    }
}
