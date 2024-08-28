//! Generates a new task every 10 seconds
use alloy::primitives::{Address, Bytes, U256};
use eigen_utils::{get_provider, get_signer};
use incredible_bindings::IncredibleSquaringTaskManager;
use lazy_static::lazy_static;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
lazy_static! {
    pub static ref TASK_NUMBER_INCREMENT_VALUE: U256 = U256::from(1);
}
use tracing::info;

///
#[derive(Debug)]
pub struct TaskManager {
    task_manager_address: Address,

    rpc_url: String,

    signer: String,
}

impl TaskManager {
    /// New [`TaskManager`] instance
    pub fn new(task_manager_address: Address, rpc_url: String, signer: String) -> Self {
        Self {
            task_manager_address,
            rpc_url,
            signer,
        }
    }

    /// Creates new task every 10 seconds
    pub async fn start(&self) -> eyre::Result<()> {
        info!("Started creating new task ");
        println!("self.signer{:?}", self.signer.clone());
        let task_manager_contract = (IncredibleSquaringTaskManager::new(
            self.task_manager_address,
            get_signer(self.signer.clone(), &self.rpc_url),
        ));

        let mut task_num: U256 = U256::from(0);

        loop {
            let number_to_be_squared = task_num;
            let quorum_threshold_percentage = 100;
            let quorum_numbers = Bytes::from_str("0x00").unwrap();
            let s = task_manager_contract
                .createNewTask(
                    number_to_be_squared,
                    quorum_threshold_percentage,
                    quorum_numbers.clone(),
                )
                .send()
                .await?
                .get_receipt()
                .await?;
            println!("receipt for task created {:?}", s.transaction_hash);

            let task_quorum = quorum_numbers.len();
            println!("quorum num length {:?}", task_quorum);
            // Increment the task number for the next iteration
            task_num += *TASK_NUMBER_INCREMENT_VALUE;
            println!("task created for num {:?}", task_num);
            // Wait for 10 seconds before the next iteration
            sleep(Duration::from_secs(10)).await;
        }
    }
}
