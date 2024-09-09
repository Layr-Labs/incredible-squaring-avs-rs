//! Builder module for the AVS. Starts all the services for the AVS using futures simulatenously.
use futures::TryFutureExt;
use incredible_aggregator::Aggregator;
use incredible_challenger::Challenger;
use incredible_config::IncredibleConfig;
use incredible_operator::builder::OperatorBuilder;
use incredible_task_generator::TaskManager;
use std::future::Future;
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

impl LaunchAvs<AvsBuilder> for DefaultAvsLauncher {
    async fn launch_avs(self, avs: AvsBuilder) -> eyre::Result<()> {
        info!("launching crates: incredible-squaring-avs-rs");
        incredible_metrics::new();
        // start operator
        let mut operator_builder = OperatorBuilder::build(avs.config.clone()).await?;
        let mut challenge = Challenger::build(avs.config.clone()).await?;
        let operator_service = operator_builder
            .start_operator()
            .map_err(|e| eyre::eyre!("Operator error: {:?}", e));

        let challenger_service = challenge
            .start_challenger()
            .map_err(|e| eyre::eyre!("Challenger error: {:?}", e));

        let aggregator = Aggregator::new(avs.config.clone()).await?;

        let aggregator_service_with_rpc_client = aggregator
            .start(avs.config.ws_rpc_url().clone())
            .map_err(|e| eyre::eyre!("Aggregator error {e:?}"));

        let task_manager = TaskManager::new(
            avs.config.task_manager_addr()?,
            avs.config.http_rpc_url(),
            avs.config.task_manager_signer(),
        );

        let task_spam_service = task_manager
            .start()
            .map_err(|e| eyre::eyre!("Task manager error {e:?}"));
        let _ = futures::future::try_join4(
            operator_service,
            challenger_service,
            aggregator_service_with_rpc_client,
            task_spam_service,
        )
        .await?;

        Ok(())
    }
}
