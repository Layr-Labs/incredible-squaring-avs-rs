//! Metrics for the incredible squaring task manager
use metrics::{counter, describe_counter};

/// Create a new instance of the metrics
pub fn new() {
    describe_counter!(
        "num_tasks_received",
        "The number of tasks received by reading from the avs service manager contract"
    );
    describe_counter!(
        "num_signed_task_responses_accepted_by_aggregator",
        "The number of signed task responses accepted by the aggregator"
    );
}

/// Increment the number of tasks received
pub fn increment_num_tasks_received() {
    counter!("num_tasks_received").increment(1);
}

/// Increment the number of tasks received
pub fn inc_num_tasks_accepted_by_aggregator() {
    counter!("num_signed_task_responses_accepted_by_aggregator").increment(1);
}

#[cfg(test)]
mod tests {

    use super::*;
    use eigen_metrics::prometheus::init_registry;
    use std::{net::SocketAddr, time::Duration};

    #[tokio::test]
    async fn test_prometheus_server() {
        let socket: SocketAddr = "127.0.0.1:9091".parse().unwrap();
        init_registry(socket);

        // Initialize  incredible squaring metrics
        new();

        tokio::time::sleep(Duration::from_secs(1)).await;

        async fn get_metrics_body(client: &reqwest::Client, url: &str) -> String {
            let resp = client.get(url).send().await.unwrap();
            resp.text().await.unwrap()
        }
        let client = reqwest::Client::new();
        let _ = client
            .get("http://127.0.0.1:9091/metrics")
            .send()
            .await
            .unwrap();

        increment_num_tasks_received();
        inc_num_tasks_accepted_by_aggregator();

        let body = get_metrics_body(&client, "http://127.0.0.1:9091/metrics").await;

        assert!(body.contains("num_tasks_received 1"));
        assert!(body.contains("num_signed_task_responses_accepted_by_aggregator 1"));
    }
}
