use futures::TryFutureExt;
use incredible_aggregator::Aggregator;
use incredible_challenger::Challenger;
use incredible_config::IncredibleConfig;
use incredible_operator::builder::OperatorBuilder;
use incredible_task_generator::TaskManager;
use std::future::{self, Future};

/// Launch Avs trait
pub trait LaunchAvs<T: Send + 'static> {
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
        println!("start launch avs");

        // start operator
        let mut operator_builder = OperatorBuilder::build(avs.config.clone())?;
        let mut challenge = Challenger::build(avs.config.clone()).await?;
        let operator_task = operator_builder
            .start_operator()
            .map_err(|e| eyre::eyre!("Operator error: {:?}", e));

        let c = challenge
            .start_challenger()
            .map_err(|e| eyre::eyre!("Challenger error: {:?}", e));

        let mut aggregator = Aggregator::new(avs.config.clone()).await;

        let a = aggregator
            .start(avs.config.ws_rpc_url().clone())
            .map_err(|e| eyre::eyre!("aggregator error {e:?}"));

        let task_manager = TaskManager::new(
            avs.config.task_manager_addr().unwrap(),
            avs.config.http_rpc_url(),
            avs.config.task_manager_signer(),
        );

        let t = task_manager
            .start()
            .map_err(|e| eyre::eyre!("task manager error {e:?}"));
        let s = futures::future::try_join4(operator_task, c, a, t).await?;

        // /// start aggregator

        println!("end");
        Ok(())
    }
}
