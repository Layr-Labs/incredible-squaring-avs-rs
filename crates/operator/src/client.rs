use reqwest::Client;



pub struct ClientAggregator{

    rpc_url:String,
    client: Client

}

impl ClientAggregator{

    pub fn new(url:String) -> Self{
        Self{
            rpc_url:url,
            client: Client::new()
        }
    }


    pub async fn send_signed_task_response(&self, signed_task_response: SignedTaskResponse) -> Result<(), reqwest::Error> {
        for _ in 0..5 {
            let response = self.client.post(&self.rpc_url)
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
                        println!("Signed task response header accepted by aggregator.");
                        // metrics here 
                        return Ok(());
                    } else {
                        println!("Received error from aggregator: {:?}", res.text().await?);
                    }
                },
                Err(err) => {
                    println!("Error sending request: {:?}", err);
                },
            }
            println!("Retrying in 2 seconds...");
            sleep(Duration::from_secs(2)).await;
        }
        println!("Could not send signed task response to aggregator. Tried 5 times.");
        Ok(())
    }
}




}