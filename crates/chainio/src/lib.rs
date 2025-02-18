//! AvsWriter

/// error
pub mod error;
/// Fake avs writer
pub mod fake_avs_writer;

use alloy::{
    primitives::{Address, U256},
    rpc::types::TransactionReceipt,
};
use eigen_common::{get_provider, get_signer};
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use eigen_utils::slashing::middleware::registrycoordinator::RegistryCoordinator::{
    self, serviceManagerReturn,
};
use error::ChainIoError;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    Task, TaskResponse, TaskResponseMetadata,
};
use incredible_bindings::{
    incrediblesquaringservicemanager::IncredibleSquaringServiceManager::{
        self, incredibleSquaringTaskManagerReturn,
    },
    incrediblesquaringtaskmanager::{
        IBLSSignatureChecker::NonSignerStakesAndSignature, IncredibleSquaringTaskManager,
        BN254::G1Point,
    },
};

use tracing::info;

/// AvsWriter struct
#[derive(Debug)]
pub struct AvsWriter {
    task_manager_addr: Address,
    signer: String,
    /// rpc url
    pub rpc_url: String,
}

impl AvsWriter {
    /// new instance
    pub async fn new(
        registry_coordinator_addr: Address,
        rpc_url: String,
        signer: String,
    ) -> Result<Self, error::ChainIoError> {
        let provider = get_provider(&rpc_url);
        let contract_registry_coordinator =
            RegistryCoordinator::new(registry_coordinator_addr, &provider);
        let service_manager_addr_return = contract_registry_coordinator
            .serviceManager()
            .call()
            .await?;
        let serviceManagerReturn {
            _0: service_manager_addr,
        } = service_manager_addr_return;

        let contract_service_manager =
            IncredibleSquaringServiceManager::new(service_manager_addr, &provider);

        let task_manager_address_return = contract_service_manager
            .incredibleSquaringTaskManager()
            .call()
            .await?;
        let incredibleSquaringTaskManagerReturn {
            _0: task_manager_address,
        } = task_manager_address_return;

        Ok(AvsWriter {
            task_manager_addr: task_manager_address,
            signer,
            rpc_url,
        })
    }

    /// Send task number to square. Return tx receipt and the task index
    pub async fn send_task_number_to_square(
        &self,
        num_to_square: U256,
        quorum_threshold_percentages: QuorumThresholdPercentage,
        quorum_nums: Vec<QuorumNum>,
    ) -> Result<(TransactionReceipt, u32), ChainIoError> {
        let wallet = get_signer(&self.signer, &self.rpc_url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(self.task_manager_addr, wallet);

        let create_new_task_call = task_manager_contract.createNewTask(
            num_to_square,
            quorum_threshold_percentages.into(),
            quorum_nums.into(),
        );

        let create_new_task_result = create_new_task_call.send().await;

        match create_new_task_result {
            Ok(create_new_task) => {
                let receipt_result = create_new_task.get_receipt().await;

                match receipt_result {
                    Ok(receipt) => {
                        let logs = &receipt.inner.logs()[0];
                        let log_decoded_option = logs
                            .log_decode::<IncredibleSquaringTaskManager::NewTaskCreated>()
                            .ok();

                        if let Some(new_task_created) = log_decoded_option {
                            let data = new_task_created.data();
                            let task_index = data.taskIndex;

                            Ok((receipt, task_index))
                        } else {
                            Err(ChainIoError::CreateNewTaskNoEventFound)
                        }
                    }

                    Err(e) => Err(ChainIoError::AlloyProviderError(e)),
                }
            }
            Err(e) => Err(ChainIoError::ContractError(e)),
        }
    }

    /// Raise challenge
    pub async fn raise_challenge(
        &self,
        task: Task,
        task_response: TaskResponse,
        task_response_metadata: TaskResponseMetadata,
        pub_keys_of_non_signing_operators: Vec<G1Point>,
    ) -> Result<alloy::rpc::types::TransactionReceipt, ChainIoError> {
        let wallet = get_signer(&self.signer, &self.rpc_url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(self.task_manager_addr, wallet);

        let challenge_tx_call = task_manager_contract.raiseAndResolveChallenge(
            task,
            task_response.clone(),
            task_response_metadata,
            pub_keys_of_non_signing_operators,
        );

        match challenge_tx_call.send().await {
            Ok(challenge_tx) => {
                let receipt_result = challenge_tx.get_receipt().await;
                match receipt_result {
                    Ok(receipts) => {
                        info!(
                            "raiseAndResolveChallenge for index{:?} tx_hash: {:?}",
                            task_response.referenceTaskIndex, receipts.transaction_hash
                        );
                        Ok(receipts)
                    }
                    Err(e) => Err(ChainIoError::AlloyProviderError(e)),
                }
            }

            Err(e) => Err(ChainIoError::ContractError(e)),
        }
    }

    /// Send aggregated response
    ///
    /// # Arguments
    ///
    /// * `task` - Task
    /// * `task_response` - Task response
    /// * `non_signer_stakes_and_signature` - Non signer stakes and signature
    pub async fn send_aggregated_response(
        &self,
        task: Task,
        task_response: TaskResponse,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> Result<(), ChainIoError> {
        let pr = get_signer(&self.signer, &self.rpc_url);
        let task_manager_contract = IncredibleSquaringTaskManager::new(self.task_manager_addr, pr);
        let receipt = task_manager_contract
            .respondToTask(task, task_response, non_signer_stakes_and_signature)
            .send()
            .await?
            .get_receipt()
            .await?;
        info!("receipt for response {:?}", receipt.transaction_hash);
        Ok(())
    }
}
