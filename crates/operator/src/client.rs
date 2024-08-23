use alloy::{
    rpc::{
        client::{ReqwestClient, RpcClient},
        types::request,
    },
    transports::{RpcError, TransportErrorKind},
};
use eyre::Result;
use incredible_aggregator::rpc_server::SignedTaskResponse;
use reqwest::Client;
use serde_json::json;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info};

/// Client Aggregator
#[derive(Debug, Clone)]
pub struct ClientAggregator {
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
        let mut delay = Duration::from_secs(1);

        for _ in 0..5 {
            let params = &json!({
                "params": [signed_task_response],
                "id": 1,
                "jsonrpc": "2.0"
            });
            let request = self
                .client
                .as_ref()
                .unwrap()
                .request("process_signed_task_response", params);

            let res: bool = request.await.unwrap();
            // match response {
            //     Ok(res) => {
            //         if res.status().is_success() {
            //             info!("Signed task response accepted by aggregator.");
            //             return Ok(());
            //         } else {
            //             info!("Received error from aggregator: {:?}", res.text().await?);
            //         }
            //     }
            //     Err(err) => {
            //         error!("Error sending request: {:?}", err);
            //     }
            // }

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

    use super::*;

    #[test]
    fn test_new_client() {
        let mut client = ClientAggregator::new("127.0.0.1:8545".to_string());
        client.dial_aggregator_rpc_client();

        println!("client {:?}", client.client.unwrap());
    }
}
