//! Generates a new task every 10 seconds
use alloy::{
    primitives::{Address, Bytes, U256},
    rpc::types::TransactionReceipt,
};
use eigen_utils::get_signer;
use incredible_bindings::IncredibleSquaringTaskManager::{
    self, NonSignerStakesAndSignature, Task, TaskResponse,
};
use lazy_static::lazy_static;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
lazy_static! {
    /// Task number increment value
    pub static ref TASK_NUMBER_INCREMENT_VALUE: U256 = U256::from(1);
}
use tracing::info;

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
        let task_manager_contract = IncredibleSquaringTaskManager::new(
            self.task_manager_address,
            get_signer(self.signer.clone(), &self.rpc_url),
        );

        let mut task_num: U256 = U256::from(0);

        loop {
            let number_to_be_squared = task_num;
            let quorum_threshold_percentage = 100;
            let quorum_numbers = Bytes::from_str("0x00")?;
            let _ = task_manager_contract
                .createNewTask(
                    number_to_be_squared,
                    quorum_threshold_percentage,
                    quorum_numbers.clone(),
                )
                .send()
                .await?
                .get_receipt()
                .await?;

            // Increment the task number for the next iteration
            task_num += *TASK_NUMBER_INCREMENT_VALUE;
            // Wait for 10 seconds before the next iteration
            sleep(Duration::from_secs(10)).await;
        }
    }

    /// Create a new task
    ///
    /// # Arguments
    ///
    /// * `task_num` - The task number to be squared
    ///
    /// # Returns
    ///
    /// A [`TransactionReceipt`]
    pub async fn create_new_task(
        &self,
        task_num: U256,
    ) -> eyre::Result<TransactionReceipt, eyre::Error> {
        let task_manager_contract = IncredibleSquaringTaskManager::new(
            self.task_manager_address,
            get_signer(self.signer.clone(), &self.rpc_url),
        );

        let number_to_be_squared = task_num;
        let quorum_threshold_percentage = 100;
        let quorum_numbers = Bytes::from_str("0x00")?;
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
        Ok(s)
    }

    /// Respond to a task
    ///
    /// # Arguments
    ///
    /// * [`task`] - The task to respond to
    /// * [`task_response`] - The response to the task
    /// * [`non_signer_stakes_and_signature`] - The non-signer stakes and signature
    ///
    /// # Returns
    ///
    /// A [`TransactionReceipt`]
    pub async fn respond_to_task(
        &self,
        task: Task,
        task_response: TaskResponse,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> eyre::Result<TransactionReceipt, eyre::Error> {
        let task_manager_contract = IncredibleSquaringTaskManager::new(
            self.task_manager_address,
            get_signer(self.signer.clone(), &self.rpc_url),
        );

        let s = task_manager_contract
            .respondToTask(task, task_response, non_signer_stakes_and_signature)
            .send()
            .await?
            .get_receipt()
            .await?;
        Ok(s)
    }
}
