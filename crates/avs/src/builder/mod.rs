use futures::TryFutureExt;
use incredible_challenger::Challenger;
use incredible_config::IncredibleConfig;
use incredible_operator::builder::OperatorBuilder;
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

        /// start operator
        let operator_builder = OperatorBuilder::build(avs.config.clone())?;
        let mut challenge = Challenger::new(avs.config).await?;
        let operator_task = operator_builder
            .start_operator()
            .map_err(|e| eyre::eyre!("Operator error: {:?}", e));

        // let c = challenge
        //     .start_challenger()
        //     .map_err(|e| eyre::eyre!("Operator error: {:?}", e));

        // let _ = futures::future::try_join(operator_task, c).await?;
        // /// start aggregator

        // println!("end");
        Ok(())
    }
}
