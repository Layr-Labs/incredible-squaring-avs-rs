//! Generates a new task every 10 seconds
use alloy::providers::ProviderBuilder;
use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, U256},
    rpc::types::TransactionReceipt,
    signers::local::PrivateKeySigner,
};
use eigen_common::get_signer;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    Task, TaskResponse,
};
use incredible_bindings::incrediblesquaringtaskmanager::{
    IBLSSignatureChecker::NonSignerStakesAndSignature,
    IncredibleSquaringTaskManager::{self},
};
use lazy_static::lazy_static;
use reqwest::Url;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
lazy_static! {
    /// Task number increment value
    pub static ref TASK_NUMBER_INCREMENT_VALUE: U256 = U256::from(1);
}

#[derive(Debug)]
pub struct TaskManager {
    task_manager_address: Address,

    rpc_url: String,

    signer: String,

    quorum_numbers: String,
}

impl TaskManager {
    /// New [`TaskManager`] instance
    pub fn new(
        task_manager_address: Address,
        rpc_url: String,
        signer: String,
        quorum_numbers: String,
    ) -> Self {
        Self {
            task_manager_address,
            rpc_url,
            signer,
            quorum_numbers,
        }
    }

    /// Creates new task every 10 seconds
    pub async fn start(&self) -> eyre::Result<()> {
        sleep(Duration::from_secs(10)).await; // wait for 10 seconds first
        let url = Url::parse(&self.rpc_url)?;
        let signer = PrivateKeySigner::from_str(&self.signer)?;
        let wallet = EthereumWallet::new(signer);
        let pr = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(self.task_manager_address, pr);
        let mut task_num: U256 = U256::from(1);

        loop {
            let number_to_be_squared = task_num;
            let quorum_threshold_percentage = 40;
            let quorum_numbers = Bytes::from_str(&self.quorum_numbers)?;

            let _ = task_manager_contract
                .createNewTask(
                    number_to_be_squared,
                    quorum_threshold_percentage,
                    quorum_numbers.clone(),
                )
                .send()
                .await?;

            // // Increment the task number for the next iteration
            task_num += *TASK_NUMBER_INCREMENT_VALUE;
            // // Wait for 10 seconds before the next iteration
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
        let url = Url::parse(&self.rpc_url)?;
        let signer = PrivateKeySigner::from_str(&self.signer)?;
        let wallet = EthereumWallet::new(signer);
        let pr = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(self.task_manager_address, pr);

        let number_to_be_squared = task_num;
        let quorum_threshold_percentage = 40;
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
        let wallet = get_signer(&self.signer, &self.rpc_url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(self.task_manager_address, wallet);

        let s = task_manager_contract
            .respondToTask(task, task_response, non_signer_stakes_and_signature)
            .send()
            .await?
            .get_receipt()
            .await?;
        Ok(s)
    }
}
