//! Builder module for the AVS. Starts all the services for the AVS using futures simulatenously.
use eigensdk::{
    aggregator::{Aggregator, AggregatorConfig},
    crypto_bls::BlsKeyPair,
    logging::get_logger,
    nodeapi::{NodeApi, NodeInfo},
    operator::Operator,
};
use futures::TryFutureExt;
use incredible_aggregator::IncredibleTaskProcessor;
use incredible_challenger::Challenger;
use incredible_config::IncredibleConfig;
use incredible_operator::OperatorTaskProcessorImpl;
use incredible_task_generator::TaskManager;
use ntex::rt::System;
use rust_bls_bn254::keystores::base_keystore::Keystore;
use std::{future::Future, time::Duration};
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
        info!("launching crates: incredible-squaring-avs-rs");
        incredible_metrics::new();

        let mut challenge = Challenger::build(avs.config.clone()).await?;
        let challenger_service = challenge
            .start_challenger()
            .map_err(|e| eyre::eyre!("Challenger error: {:?}", e));

        // Start the aggregator
        let aggregator_config = AggregatorConfig {
            server_address: avs.config.aggregator_ip_addr(),
            registry_coordinator: avs.config.registry_coordinator_addr()?,
            operator_state_retriever: avs.config.operator_state_retriever_addr()?,
            http_rpc_url: avs.config.http_rpc_url(),
            ws_rpc_url: avs.config.ws_rpc_url(),
        };
        let task_processor = IncredibleTaskProcessor::new(avs.config.clone())
            .await
            .map_err(|e| eyre::eyre!("Task processor error: {:?}", e))?;
        let aggregator = Aggregator::new(aggregator_config, task_processor)
            .await
            .map_err(|e| eyre::eyre!("Aggregator new error {e:?}"))?;

        let ws_rpc_url = avs.config.ws_rpc_url();

        // Need to start the aggregator here since it needs to be started before the operators
        tokio::spawn(async move {
            let _ = aggregator
                .start(ws_rpc_url)
                .map_err(|e| eyre::eyre!("Aggregator start error: {e:?}"))
                .await;
        });

        // Sleep for 10 seconds to ensure the aggregator is started
        tokio::time::sleep(Duration::from_secs(10)).await;

        // Register and start both operators
        let keystore = Keystore::from_file(&avs.config.bls_keystore_path())?
            .decrypt(&avs.config.bls_keystore_password())
            .unwrap();
        let fr_key: String = keystore.iter().map(|&value| value as char).collect();
        let bls_key_pair = BlsKeyPair::new(fr_key)?;
        let operator_task_processor = OperatorTaskProcessorImpl::new(
            avs.config.operator_1_times_failing().unwrap_or_default(),
        );
        let operator_1_address = avs.config.operator_address()?;

        let operator = Operator::new(
            &bls_key_pair,
            operator_1_address,
            "FIRST OPERATOR",
            get_logger(),
            &avs.config.ws_rpc_url(),
            &avs.config.http_rpc_url(),
            avs.config.registry_coordinator_addr()?,
            avs.config.operator_state_retriever_addr()?,
            avs.config.aggregator_ip_addr(),
            operator_task_processor.clone(),
        )
        .await
        .unwrap();

        let operator_1_service = operator
            .start()
            .map_err(|e| eyre::eyre!("Operator 1 start error {e:?}"));

        let keystore = Keystore::from_file(&avs.config.bls_keystore_2_path())?
            .decrypt(&avs.config.bls_keystore_2_password())
            .unwrap();
        let fr_key: String = keystore.iter().map(|&value| value as char).collect();
        let bls_key_pair = BlsKeyPair::new(fr_key)?;
        let operator_2_task_processor = OperatorTaskProcessorImpl::new(
            avs.config.operator_2_times_failing().unwrap_or_default(),
        );
        let operator_2_address = avs.config.operator_2_address()?;

        let operator_2 = Operator::new(
            &bls_key_pair,
            operator_2_address,
            "SECOND OPERATOR",
            get_logger(),
            &avs.config.ws_rpc_url(),
            &avs.config.http_rpc_url(),
            avs.config.registry_coordinator_addr()?,
            avs.config.operator_state_retriever_addr()?,
            avs.config.aggregator_ip_addr(),
            operator_2_task_processor,
        )
        .await
        .unwrap();

        let operator_2_service = operator_2
            .start()
            .map_err(|e| eyre::eyre!("Operator 2 start error {e:?}"));

        let task_manager = TaskManager::new(
            avs.config.task_manager_addr()?,
            avs.config.http_rpc_url(),
            avs.config.task_manager_signer(),
            avs.config.quorum_number()?.to_string(),
        );
        let task_spam_service = task_manager
            .start()
            .map_err(|e| eyre::eyre!("Task manager error {e:?}"));
        let node_info = NodeInfo::new("incredible-squaring", "v0.0.1");
        let node_api = NodeApi::new(node_info);
        let node_api_address = avs.config.node_api_port_address();
        info!("node_api_address{:?}", node_api_address);

        std::thread::spawn(move || {
            let _ = System::new("node_api_system").block_on(async move {
                let node_api_server = node_api.start_server(&node_api_address).unwrap();
                node_api_server.await
            });
        });

        let _ = futures::future::try_join4(
            challenger_service,
            operator_1_service,
            operator_2_service,
            task_spam_service,
        )
        .await?;

        Ok(())
    }
}
