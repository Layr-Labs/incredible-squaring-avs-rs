//! AvsWriter

/// error
pub mod error;

use alloy::{
    primitives::{Address, U256},
    rpc::types::TransactionReceipt,
};
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use eigen_utils::{
    binding::RegistryCoordinator::{self, serviceManagerReturn},
    get_provider, get_signer,
};
use error::ChainIoError;
use incredible_bindings::{
    IncredibleSquaringServiceManager::{self, incredibleSquaringTaskManagerReturn},
    IncredibleSquaringTaskManager::{self, G1Point, Task, TaskResponse, TaskResponseMetadata},
};

/// AvsWriter struct
#[derive(Debug)]
pub struct AvsWriter {
    task_manager_addr: Address,
    signer: String,
    rpc_url: String,
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
        println!("4444");
        let service_manager_addr_return = contract_registry_coordinator
            .serviceManager()
            .call()
            .await?;
        let serviceManagerReturn {
            _0: service_manager_addr,
        } = service_manager_addr_return;

        let contract_service_manager =
            IncredibleSquaringServiceManager::new(service_manager_addr, &provider);

        println!("service manager addr {}", service_manager_addr);

        let task_manager_address_return = contract_service_manager
            .incredibleSquaringTaskManager()
            .call()
            .await?;
        let incredibleSquaringTaskManagerReturn {
            _0: task_manager_address,
        } = task_manager_address_return;
        println!("task manager addr {}", service_manager_addr);

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
        let signer = get_signer(self.signer.clone(), &self.rpc_url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(self.task_manager_addr, signer);

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

                    Err(e) => Err(ChainIoError::AlloyContractError(
                        alloy::contract::Error::TransportError(e),
                    )),
                }
            }
            Err(e) => Err(ChainIoError::AlloyContractError(e)),
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
        let signer = get_signer(self.signer.clone(), &self.rpc_url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(self.task_manager_addr, signer);

        let challenge_tx_call = task_manager_contract.raiseAndResolveChallenge(
            task,
            task_response,
            task_response_metadata,
            pub_keys_of_non_signing_operators,
        );

        match challenge_tx_call.send().await {
            Ok(challenge_tx) => {
                let receipt_result = challenge_tx.get_receipt().await;

                match receipt_result {
                    Ok(receipts) => Ok(receipts),
                    Err(e) => Err(ChainIoError::AlloyContractError(
                        alloy::contract::Error::TransportError(e),
                    )),
                }
            }

            Err(e) => Err(ChainIoError::AlloyContractError(e)),
        }
    }
}
