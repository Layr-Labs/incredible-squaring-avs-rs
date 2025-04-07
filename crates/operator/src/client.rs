use eigensdk::aggregator::{rpc_server::ProcessSignedTaskResponseClient, SignedTaskResponse};
use eyre::Result;
use incredible_aggregator::IncredibleTaskResponse;
use tarpc::tokio_serde::formats::Json;
use tokio::time::Duration;
use tracing::{debug, info};

/// Client Aggregator
#[derive(Debug, Clone)]
pub struct ClientAggregator {
    /// Alloy rpc client to send requests to aggregator
    pub client: ProcessSignedTaskResponseClient,
}

impl ClientAggregator {
    /// new
    pub async fn new(aggregator_ip_port_address: String) -> Result<Self> {
        let transport =
            tarpc::serde_transport::tcp::connect(aggregator_ip_port_address, Json::default).await?;
        let client =
            ProcessSignedTaskResponseClient::new(tarpc::client::Config::default(), transport)
                .spawn();
        Ok(Self { client })
    }

    /// Send signed task response
    pub async fn send_signed_task_response(
        &self,
        signed_task_response: SignedTaskResponse<IncredibleTaskResponse>,
    ) -> Result<()> {
        #[allow(unused_mut)]
        let mut delay = Duration::from_secs(1);

        for _ in 0..5 {
            let ctx = tarpc::context::current();
            let params = serde_json::to_string(&signed_task_response)?;
            let response = self
                .client
                .process_signed_task_response(ctx, params)
                .await??;

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

mod tests {

    #[tokio::test]
    async fn test_new_client() {
        crate::client::ClientAggregator::new("127.0.0.1:8545".to_string())
            .await
            .unwrap();
    }
}
