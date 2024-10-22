use std::str::FromStr;

use crate::error::ChainIoError;
use alloy::signers::local::PrivateKeySigner;
use alloy::{
    network::EthereumWallet,
    primitives::{Address, TxHash},
    providers::ProviderBuilder,
};
use incredible_bindings::incrediblesquaringtaskmanager::{
    IBLSSignatureChecker::NonSignerStakesAndSignature,
    IIncredibleSquaringTaskManager::{Task, TaskResponse, TaskResponseMetadata},
    IncredibleSquaringTaskManager,
    BN254::G1Point,
};
use reqwest::Url;

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
        let url = Url::parse(&self.rpc_url).expect("Wrong rpc url");
        let signer = PrivateKeySigner::from_str(&self.signer)?;
        let wallet = EthereumWallet::new(signer);
        let pr = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(url);
        let task_manager_contract = IncredibleSquaringTaskManager::new(self.task_manager_addr, pr);

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
        let url = Url::parse(&self.rpc_url).expect("Wrong rpc url");
        let signer = PrivateKeySigner::from_str(&self.signer)?;
        let wallet = EthereumWallet::new(signer);
        let pr = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(url);
        let task_manager_contract = IncredibleSquaringTaskManager::new(self.task_manager_addr, pr);

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
