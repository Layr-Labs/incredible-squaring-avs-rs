use alloy::rpc::client::{ReqwestClient, RpcClient};
use eyre::Result;
pub use incredible_aggregator::rpc_server::SignedTaskResponse;
use reqwest::Client;
use serde_json::json;
use tokio::time::{sleep, Duration};
use tracing::{debug, info};

/// Client Aggregator
#[derive(Debug, Clone)]
pub struct ClientAggregator {
    /// Alloy rpc client to send requests to aggregator
    pub client: Option<RpcClient<alloy::transports::http::Http<Client>>>,
    aggregator_ip_port_address: String,
}

impl ClientAggregator {
    /// new
    pub fn new(aggregator_ip_port_address: String) -> Self {
        Self {
            client: None,
            aggregator_ip_port_address,
        }
    }

    /// new http rpc client instance using the aggregator ip port address
    pub fn dial_aggregator_rpc_client(&mut self) {
        let url =
            reqwest::Url::parse(&format!("http://{}", &self.aggregator_ip_port_address)).unwrap();
        let client = ReqwestClient::new_http(url);

        self.client = Some(client)
    }

    /// Send signed task response
    pub async fn send_signed_task_response(
        &self,
        signed_task_response: SignedTaskResponse,
    ) -> Result<()> {
        info!("Received signed task response");
        #[allow(unused_mut)]
        let mut delay = Duration::from_secs(1);

        for _ in 0..5 {
            let params = &json!({
                "params": signed_task_response,
                "id": 1,
                "jsonrpc": "2.0"
            });
            let request = self
                .client
                .as_ref()
                .unwrap()
                .request("process_signed_task_response", params)
                .await?;

            if request {
                incredible_metrics::inc_num_tasks_accepted_by_aggregator();
                return Ok(());
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

mod tests {

    #[test]
    fn test_new_client() {
        let mut client = crate::client::ClientAggregator::new("127.0.0.1:8545".to_string());
        client.dial_aggregator_rpc_client();
    }
}
