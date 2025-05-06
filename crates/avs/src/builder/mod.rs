//! Builder module for the AVS. Starts all the services for the AVS using futures simulatenously.
use alloy::{
    network::EthereumWallet, primitives::U256, providers::ProviderBuilder,
    signers::local::PrivateKeySigner, transports::http::reqwest::Url,
};
use eigensdk::{
    aggregator::{Aggregator, AggregatorConfig},
    challenger::{challenger_processor::IndexingChallengerProcessor, Challenger},
    crypto_bls::BlsKeyPair,
    logging::get_logger,
    nodeapi::{NodeApi, NodeInfo},
    operator::{config::OperatorConfig, with_failures, Operator},
    task_processor::IndexingTaskProcessor,
    task_spammer::TaskSpammerBuilder,
};
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager;
use incredible_challenger::is_response_correct;
use incredible_config::IncredibleConfig;
use incredible_operator::{square, square_with_failure};
use ntex::rt::System;
use rust_bls_bn254::keystores::base_keystore::Keystore;
use std::{future::Future, str::FromStr, time::Duration};
use tracing::info;

/// Task Challenge Window Block : 100 blocks
pub const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
/// Block Time Seconds : 12 seconds
pub const BLOCK_TIME_SECONDS: u32 = 12;

use crate::TaskManagerWrapper;
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
    async fn launch_avs(self, mut avs: AvsBuilder) -> eyre::Result<()> {
        // General setup
        info!("launching crates: incredible-squaring-avs-rs");
        incredible_metrics::new();
        let logger = get_logger();
        let http_rpc_url = avs.config.http_rpc_url().to_string();
        let ws_rpc_url = avs.config.ws_rpc_url().to_string();
        let registry_coordinator = avs.config.registry_coordinator_addr()?;
        let operator_state_retriever = avs.config.operator_state_retriever_addr()?;

        // Create task manager contract instance for aggregator
        let url = Url::parse(&avs.config.http_rpc_url())?;
        let signer = PrivateKeySigner::from_str(&avs.config.get_signer())?;
        let wallet = EthereumWallet::new(signer);
        let pr = ProviderBuilder::new().wallet(wallet).on_http(url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(avs.config.task_manager_addr()?, pr);
        let task_manager_wrapper = TaskManagerWrapper(task_manager_contract);

        // Create task manager contract instance for task spammer
        // Task spammer uses a different signer than the aggregator
        avs.config.set_signer(
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".to_string(),
        );
        // Create task manager contract instance
        let url = Url::parse(&avs.config.http_rpc_url())?;
        let signer = PrivateKeySigner::from_str(&avs.config.get_signer())?;
        let wallet = EthereumWallet::new(signer);
        let pr = ProviderBuilder::new().wallet(wallet).on_http(url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(avs.config.task_manager_addr()?, pr);
        let task_manager_wrapper_spammer = TaskManagerWrapper(task_manager_contract);

        // ==================== Launch aggregator ====================
        info!("Launching aggregator");
        let config_aggregator = AggregatorConfig {
            http_rpc_url: http_rpc_url.clone(),
            ws_rpc_url: ws_rpc_url.clone(),
            server_address: avs.config.aggregator_ip_addr().to_string(),
            registry_coordinator,
            operator_state_retriever,
        };

        let time_to_expiry = tokio::time::Duration::from_secs(
            (TASK_CHALLENGE_WINDOW_BLOCK * BLOCK_TIME_SECONDS).into(),
        );

        let aggregator_task_processor = IndexingTaskProcessor::new(
            task_manager_wrapper.clone(),
            time_to_expiry,
            Duration::from_secs(5),
        );

        let aggregator = Aggregator::new(config_aggregator, aggregator_task_processor)
            .await
            .map_err(|e| eyre::eyre!("Aggregator error {e:?}"))?;
        let aggregator_handle = tokio::spawn(async move {
            aggregator
                .start()
                .await
                .map_err(|e| eyre::eyre!("Aggregator error {e:?}"))
        });

        info!("Aggregator launched");

        // ==================== Launch challenger ====================
        info!("Launching challenger");
        let challenger_task_processor =
            IndexingChallengerProcessor::new(task_manager_wrapper.clone(), is_response_correct);
        let mut challenger = Challenger::new(
            http_rpc_url.clone(),
            ws_rpc_url.clone(),
            challenger_task_processor,
        );

        let challenger_handle = tokio::spawn(async move {
            challenger
                .start_challenger()
                .await
                .map_err(|e| eyre::eyre!("Challenger error {e:?}"))
        });

        info!("Challenger launched");

        info!("Sleeping for 10 seconds to ensure aggregator and challenger are running");
        tokio::time::sleep(Duration::from_secs(10)).await;

        // ==================== Launch Operators ====================
        info!("Launching operators");

        // First operator
        info!("Launching first operator");
        let keystore = Keystore::from_file(&avs.config.bls_keystore_path())?
            .decrypt(&avs.config.bls_keystore_password())?;
        let fr_key: String = keystore.iter().map(|&value| value as char).collect();
        let first_bls_key_pair = BlsKeyPair::new(fr_key)?;
        let first_operator_address = avs.config.operator_address()?;

        let operator_config = OperatorConfig {
            aggregator_ip_port: avs.config.aggregator_ip_addr().to_string(),
            bls_key_pair: first_bls_key_pair,
            operator_address: first_operator_address,
            operator_name: "FIRST OPERATOR NAME".to_string(),
            ws_rpc_url: ws_rpc_url.clone(),
            http_rpc_url: http_rpc_url.clone(),
            registry_coordinator_address: registry_coordinator,
            operator_state_retriever_address: operator_state_retriever,
        };

        let first_operator = Operator::new(logger.clone(), operator_config).await?;
        let response_logic = with_failures(square, square_with_failure, 100);

        let first_operator_handle = tokio::spawn(async move {
            first_operator
                .start(response_logic)
                .await
                .map_err(|e| eyre::eyre!("Operator error {e:?}"))
        });

        info!("First operator launched");

        // Second operator
        info!("Launching second operator");
        let keystore = Keystore::from_file(&avs.config.bls_keystore_2_path())?
            .decrypt(&avs.config.bls_keystore_2_password())?;
        let fr_key: String = keystore.iter().map(|&value| value as char).collect();
        let second_bls_key_pair = BlsKeyPair::new(fr_key)?;
        let second_operator_address = avs.config.operator_2_address()?;

        let second_operator_config = OperatorConfig {
            aggregator_ip_port: avs.config.aggregator_ip_addr().to_string(),
            bls_key_pair: second_bls_key_pair,
            operator_address: second_operator_address,
            operator_name: "SECOND OPERATOR NAME".to_string(),
            ws_rpc_url: ws_rpc_url.clone(),
            http_rpc_url: http_rpc_url.clone(),
            registry_coordinator_address: registry_coordinator,
            operator_state_retriever_address: operator_state_retriever,
        };
        let second_operator = Operator::new(logger.clone(), second_operator_config).await?;

        let response_logic = with_failures(square, square_with_failure, 10);

        let second_operator_handle = tokio::spawn(async move {
            second_operator
                .start(response_logic)
                .await
                .map_err(|e| eyre::eyre!("Operator error {e:?}"))
        });

        info!("Second operator launched");

        info!("Sleeping for 10 seconds to ensure operators are running");
        tokio::time::sleep(Duration::from_secs(10)).await;

        // ==================== Launch Task Spammer ====================
        info!("Launching task spammer");

        TaskSpammerBuilder::new(task_manager_wrapper_spammer)
            .with_iter((0..).map(U256::from))
            .with_quorum(50, vec![0])
            .with_interval(Duration::from_secs(10))
            .build()
            .map_err(|e| eyre::eyre!("Task spammer builder error {e:?}"))?
            .run()
            .await
            .map_err(|e| eyre::eyre!("Task spammer error {e:?}"))?;

        info!("Task spammer launched");

        // ==================== Launch Node API ====================
        info!("Launching node api");
        let node_info = NodeInfo::new("incredible-squaring", "v0.0.1");
        let node_api = NodeApi::new(node_info);
        let node_api_address = avs.config.node_api_port_address();
        info!("node_api_address{:?}", node_api_address);

        std::thread::spawn(move || {
            let _ = System::new("node_api_system").block_on(async move {
                let node_api_server = node_api.start_server(node_api_address.as_str()).unwrap();
                node_api_server.await
            });
        });
        info!("Node api launched");

        let _ = futures::future::try_join4(
            aggregator_handle,
            challenger_handle,
            first_operator_handle,
            second_operator_handle,
        )
        .await
        .unwrap();

        Ok(())
    }
}
