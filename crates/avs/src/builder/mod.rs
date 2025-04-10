//! Builder module for the AVS. Starts all the services for the AVS using futures simulatenously.
use eigensdk::aggregator::{Aggregator, AggregatorConfig};
use eigensdk::crypto_bls::BlsKeyPair;
use eigensdk::logging::get_logger;
use eigensdk::nodeapi::{NodeApi, NodeInfo};
use eigensdk::operator::Operator;
use eigensdk::testing_utils::anvil_constants::{FIRST_ADDRESS, OPERATOR_BLS_KEY, SECOND_ADDRESS};
use futures::TryFutureExt;
use incredible_aggregator::IncredibleTaskProcessor;
use incredible_challenger::Challenger;
use incredible_config::IncredibleConfig;
use incredible_operator::OperatorTaskProcessorImpl;
use incredible_task_generator::TaskManager;
use ntex::rt::System;
use rust_bls_bn254::keystores::base_keystore::Keystore;
use std::{future::Future, sync::Arc};
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
        // start operator

        let mut challenge = Challenger::build(avs.config.clone()).await?;
        let challenger_service = challenge
            .start_challenger()
            .map_err(|e| eyre::eyre!("Challenger error: {:?}", e));
        let ws_rpc_url = avs.config.ws_rpc_url();
        let aggregator_config = AggregatorConfig {
            server_address: avs.config.aggregator_ip_addr(),
            registry_coordinator: avs
                .config
                .registry_coordinator_addr()
                .map_err(|e| eyre::eyre!("Registry coordinator error: {:?}", e))?,
            operator_state_retriever: avs
                .config
                .operator_state_retriever_addr()
                .map_err(|e| eyre::eyre!("Operator state retriever error: {:?}", e))?,
            http_rpc_url: avs.config.http_rpc_url(),
            ws_rpc_url: ws_rpc_url.clone(),
        };
        let task_processor = IncredibleTaskProcessor::new(avs.config.clone())
            .await
            .map_err(|e| eyre::eyre!("Task processor error: {:?}", e))?;
        let aggregator = Aggregator::new(aggregator_config, task_processor)
            .await
            .map_err(|e| eyre::eyre!("Aggregator new error {e:?}"))?;
        tokio::spawn(async move {
            aggregator.start(ws_rpc_url.clone()).await.unwrap();
        });

        dbg!("WAITING TO AGGREGATOR");
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        let server_address = avs.config.aggregator_ip_addr();
        let ws_rpc_url = avs.config.ws_rpc_url();
        let http_rpc_url = avs.config.http_rpc_url();
        let registry_coordinator = avs.config.registry_coordinator_addr()?;
        let operator_state_retriever = avs.config.operator_state_retriever_addr()?;
        let keystore = Keystore::from_file(&avs.config.bls_keystore_path())?
            .decrypt(&avs.config.bls_keystore_password())
            .unwrap();
        let fr_key: String = keystore.iter().map(|&value| value as char).collect();
        let bls_key_pair = BlsKeyPair::new(fr_key)?;
        let operator_task_processor = OperatorTaskProcessorImpl;
        let operator_1_address = avs.config.operator_address()?;

        let operator = Operator::new(
            &bls_key_pair,
            operator_1_address,
            "FIRST OPERATOR",
            get_logger(),
            &ws_rpc_url,
            &http_rpc_url,
            registry_coordinator,
            operator_state_retriever,
            server_address.clone(),
            operator_task_processor.clone(),
        )
        .await
        .unwrap();

        tokio::spawn(async move {
            operator.start().await.unwrap();
        });

        let keystore = Keystore::from_file(&avs.config.bls_keystore_2_path())?
            .decrypt(&avs.config.bls_keystore_2_password())
            .unwrap();
        let fr_key: String = keystore.iter().map(|&value| value as char).collect();
        let bls_key_pair = BlsKeyPair::new(fr_key)?;
        let operator_2_address = avs.config.operator_2_address()?;

        let operator_2 = Operator::new(
            &bls_key_pair,
            operator_2_address,
            "SECOND OPERATOR",
            get_logger(),
            &ws_rpc_url,
            &http_rpc_url,
            registry_coordinator,
            operator_state_retriever,
            server_address,
            operator_task_processor,
        )
        .await
        .unwrap();

        tokio::spawn(async move {
            operator_2.start().await.unwrap();
        });

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

        let _ = futures::future::try_join(challenger_service, task_spam_service).await?;

        Ok(())
    }
}
