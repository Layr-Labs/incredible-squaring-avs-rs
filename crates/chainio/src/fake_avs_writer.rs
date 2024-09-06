use alloy::primitives::{Address, TxHash};
use eigen_utils::get_signer;
use incredible_bindings::IncredibleSquaringTaskManager::{
    self, G1Point, Task, TaskResponse, TaskResponseMetadata,
};

use crate::error::ChainIoError;

/// AvsWriter struct
#[derive(Debug, Clone)]
pub struct FakeAvsWriter {
    /// Task manager address
    pub task_manager_addr: Address,
    /// Signer
    pub signer: String,
    /// Rpc url
    pub rpc_url: String,
}

impl FakeAvsWriter {
    /// Raise challenge
    pub async fn raise_challenge(
        &self,
        task: Task,
        task_response: TaskResponse,
        task_response_metadata: TaskResponseMetadata,
        pub_keys_of_non_signing_operators: Vec<G1Point>,
    ) -> Result<TxHash, ChainIoError> {
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
                    Ok(receipts) => Ok(receipts.transaction_hash),
                    Err(e) => Err(ChainIoError::AlloyContractError(
                        alloy::contract::Error::TransportError(e),
                    )),
                }
            }

            Err(_) => {
                // return fake hash
                let tx_hash = TxHash::default();
                Ok(tx_hash)
            }
        }
    }

    pub async fn send_aggregated_response(
        &self,
        task: Task,
        task_response: TaskResponse,
        non_signer_stakes_and_signature: IncredibleSquaringTaskManager::NonSignerStakesAndSignature,
    ) {
        println!("sending_aggregated response");
        let signer = get_signer(self.signer.clone(), &self.rpc_url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(self.task_manager_addr, signer);

        let _ = task_manager_contract
            .respondToTask(task, task_response, non_signer_stakes_and_signature)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
    }
}
