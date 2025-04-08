use std::time::Duration;

use eigensdk::aggregator::{rpc_server::ProcessSignedTaskResponseClient, SignedTaskResponse};
use eyre::Result;
use incredible_aggregator::IncredibleTaskResponse;
use tarpc::tokio_serde::formats::Json;
use tracing::{debug, info};

/// Client Aggregator
#[derive(Debug, Clone)]
pub struct ClientAggregator {
    aggregator_ip_port_address: String,
}

impl ClientAggregator {
    /// new
    pub fn new(aggregator_ip_port_address: String) -> Self {
        Self {
            aggregator_ip_port_address,
        }
    }

    /// Send signed task response
    pub async fn send_signed_task_response(
        &self,
        signed_task_response: SignedTaskResponse<IncredibleTaskResponse>,
    ) -> Result<()> {
        let mut delay = Duration::from_secs(1);

        let transport = tarpc::serde_transport::tcp::connect(
            self.aggregator_ip_port_address.clone(),
            Json::default,
        )
        .await?;
        let client =
            ProcessSignedTaskResponseClient::new(tarpc::client::Config::default(), transport)
                .spawn();

        for _ in 0..5 {
            let ctx = tarpc::context::current();
            let params = serde_json::to_string(&signed_task_response)?;
            let response = client.process_signed_task_response(ctx, params).await??;

            if response {
                incredible_metrics::inc_num_tasks_accepted_by_aggregator();
                return Ok(());
            }

            // Exponential backoff
            info!("Retrying in {} seconds...", delay.as_secs());
            tokio::time::sleep(delay).await;
            delay *= 2; // Double the delay for the next retry
        }

        debug!("Could not send signed task response to aggregator. Tried 5 times.");
        Ok(())
    }
}
