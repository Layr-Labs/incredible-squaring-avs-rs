use eyre::Result;
use incredible_aggregator::SignedTaskResponse;
use reqwest::Client;
use serde_json::json;
use tokio::time::{sleep, Duration};

use tracing::{debug, error, info};

/// Client Aggregator
#[derive(Debug, Clone)]
pub struct ClientAggregator {
    rpc_url: String,
    client: Client,
}

impl ClientAggregator {
    /// new
    pub fn new(url: String) -> Self {
        Self {
            rpc_url: url,
            client: Client::new(),
        }
    }

    /// Send signed task response
    pub async fn send_signed_task_response(
        &self,
        signed_task_response: SignedTaskResponse,
    ) -> Result<()> {
        let mut delay = Duration::from_secs(1);

        for _ in 0..5 {
            let response = self
                .client
                .post(&self.rpc_url)
                .json(&json!({
                    "method": "Aggregator.ProcessSignedTaskResponse",
                    "params": [signed_task_response],
                    "id": 1,
                    "jsonrpc": "2.0"
                }))
                .send()
                .await;

            match response {
                Ok(res) => {
                    if res.status().is_success() {
                        info!("Signed task response accepted by aggregator.");
                        return Ok(());
                    } else {
                        info!("Received error from aggregator: {:?}", res.text().await?);
                    }
                }
                Err(err) => {
                    error!("Error sending request: {:?}", err);
                }
            }

            // Exponential backoff
            info!("Retrying in {} seconds...", delay.as_secs());
            sleep(delay).await;
            delay *= 2; // Double the delay for the next retry
        }

        debug!("Could not send signed task response to aggregator. Tried 5 times.");
        Ok(())
    }
}
