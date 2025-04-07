use alloy::rpc::client::{ReqwestClient, RpcClient};
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
    pub client: Option<RpcClient>,
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
    pub fn dial_aggregator_rpc_client(&mut self) -> Result<()> {
        let url = reqwest::Url::parse(&format!("http://{}", &self.aggregator_ip_port_address))?;
        let client = ReqwestClient::new_http(url);

        self.client = Some(client);
        Ok(())
    }

    /// Send signed task response
    pub async fn send_signed_task_response(
        &self,
        signed_task_response: SignedTaskResponse<IncredibleTaskResponse>,
    ) -> Result<()> {
        #[allow(unused_mut)]
        let mut delay = Duration::from_secs(1);

        // for _ in 0..5 {
        //     if let Some(request) = self.client.as_ref() {
        //         let ctx = tarpc::context::current();
        //         let params = serde_json::to_string(&signed_task_response)?;
        //         let response = request.process_signed_task_response(ctx, params).await??;

        //         if response {
        //             incredible_metrics::inc_num_tasks_accepted_by_aggregator();
        //             return Ok(());
        //         }

        //         // Exponential backoff
        //         info!("Retrying in {} seconds...", delay.as_secs());
        //         sleep(delay).await;
        //         delay *= 2; // Double the delay for the next retry
        //     }
        // }

        let transport =
            tarpc::serde_transport::tcp::connect("127.0.0.1:8080", Json::default).await?;
        let client =
            ProcessSignedTaskResponseClient::new(tarpc::client::Config::default(), transport)
                .spawn();
        let ctx = tarpc::context::current();
        let params = serde_json::to_string(&signed_task_response)?;
        let response = client.process_signed_task_response(ctx, params).await??;

        if response {
            incredible_metrics::inc_num_tasks_accepted_by_aggregator();

            return Ok(());
        }

        debug!("Could not send signed task response to aggregator. Tried 5 times.");
        Ok(())
    }
}

mod tests {

    #[test]
    fn test_new_client() {
        let mut client = crate::client::ClientAggregator::new("127.0.0.1:8545".to_string());
        let _ = client.dial_aggregator_rpc_client();
    }
}
