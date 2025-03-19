use crate::error::ChainIoError;
use alloy::primitives::{Address, TxHash};
use eigensdk::common::get_signer;
use incredible_bindings::incrediblesquaringtaskmanager::{
    IBLSSignatureChecker::NonSignerStakesAndSignature,
    IIncredibleSquaringTaskManager::{Task, TaskResponse, TaskResponseMetadata},
    IncredibleSquaringTaskManager,
    BN254::G1Point,
};

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
        let wallet = get_signer(&self.signer, &self.rpc_url);

        let task_manager_contract =
            IncredibleSquaringTaskManager::new(self.task_manager_addr, wallet);

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
                    Err(e) => Err(ChainIoError::AlloyProviderError(e)),
                }
            }

            Err(_) => {
                // return fake hash
                let tx_hash = TxHash::default();
                Ok(tx_hash)
            }
        }
    }

    /// Send the aggregated response
    /// task -  [`Task`]
    /// task_response - [`TaskResponse`]
    /// non_signer_stakes_and_signature - [`NonSignerStakesAndSignature`]
    pub async fn send_aggregated_response(
        &self,
        task: Task,
        task_response: TaskResponse,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> eyre::Result<()> {
        let wallet = get_signer(&self.signer, &self.rpc_url);
        let task_manager_contract =
            IncredibleSquaringTaskManager::new(self.task_manager_addr, wallet);

        let _ = task_manager_contract
            .respondToTask(task, task_response, non_signer_stakes_and_signature)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await;
        Ok(())
    }
}
