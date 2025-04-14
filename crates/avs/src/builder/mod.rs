//! Builder module for the AVS. Starts all the services for the AVS using futures simulatenously.
use alloy::{
    network::EthereumWallet, primitives::U256, providers::ProviderBuilder,
    signers::local::PrivateKeySigner, transports::http::reqwest::Url,
};
use eigensdk::{
    aggregator::{Aggregator, AggregatorConfig},
    crypto_bls::BlsKeyPair,
    logging::get_logger,
    nodeapi::{NodeApi, NodeInfo},
    operator::Operator,
    task_generator::TaskGenerator,
};
use futures::TryFutureExt;
use incredible_aggregator::IncredibleTaskProcessor;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager;
use incredible_challenger::Challenger;
use incredible_config::IncredibleConfig;
use incredible_operator::OperatorTaskProcessorImpl;
use ntex::rt::System;
use rust_bls_bn254::keystores::base_keystore::Keystore;
use std::{future::Future, str::FromStr, time::Duration};
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

        // Need to start the aggregator here since it needs to be started before the operators
        tokio::spawn(async move {
            let _ = aggregator
                .start()
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

        // Start the task generator service
        let url = Url::parse(&avs.config.http_rpc_url())?;
        dbg!(&avs.config.task_manager_signer());
        let signer = PrivateKeySigner::from_str(&avs.config.task_manager_signer())?;
        let wallet = EthereumWallet::new(signer);
        let pr = ProviderBuilder::new().wallet(wallet).on_http(url);
        let contract = IncredibleSquaringTaskManager::new(avs.config.task_manager_addr()?, pr);

        let task_spam_service = TaskGenerator::builder()
            .with_iter(0..)
            .with_quorum(70, vec![0])
            .with_interval(Duration::from_secs(10))
            .run(move |i, quorum_threshold, quorums| {
                let contract = contract.clone();
                async move {
                    info!("Creating task with index {i}");
                    let number_to_be_squared = U256::from(i * i);
                    contract
                        .createNewTask(
                            number_to_be_squared,
                            quorum_threshold.into(),
                            quorums.into(),
                        )
                        .send()
                        .await
                        .unwrap()
                        .get_receipt()
                        .await
                        .unwrap();

                    info!("Task {} created", i);
                    Ok(())
                }
            })
            .map_err(|e| eyre::eyre!("Task spam service error: {:?}", e));

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
