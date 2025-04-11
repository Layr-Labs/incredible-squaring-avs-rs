use eigensdk::aggregator::{rpc_server::ProcessSignedTaskResponseClient, SignedTaskResponse};
use eyre::Result;
use incredible_aggregator::IncredibleTaskResponse;
use tarpc::tokio_serde::formats::Json;
use tracing::info;

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
        let transport = tarpc::serde_transport::tcp::connect(
            self.aggregator_ip_port_address.clone(),
            Json::default,
        )
        .await?;
        let client =
            ProcessSignedTaskResponseClient::new(tarpc::client::Config::default(), transport)
                .spawn();

        let ctx = tarpc::context::current();
        let params = serde_json::to_string(&signed_task_response)?;
        let response = client.process_signed_task_response(ctx, params).await??;

        if response {
            info!("Signed task response sent to aggregator");
            incredible_metrics::inc_num_tasks_accepted_by_aggregator();
            return Ok(());
        }

        Err(eyre::eyre!(
            "Could not send signed task response to aggregator"
        ))
    }
}
