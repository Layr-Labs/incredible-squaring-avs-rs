//! Starts all the services for the AVS using futures
pub mod builder;

use alloy::{
    contract::private::{Provider, Transport},
    network::{Network, ReceiptResponse},
    primitives::U256,
};
use eigensdk::{
    challenger::{challenger_processor::TaskResponseMetadataSol, task_manager::TaskManagerError},
    task_processor::{task::Task, task_response::TaskResponse},
    types::operator::{QuorumNum, QuorumThresholdPercentage},
    utils::slashing::middleware::iblssignaturechecker::{
        IBLSSignatureCheckerTypes::NonSignerStakesAndSignature, BN254::G1Point as G1PointSDK,
    },
};
use incredible_bindings::incrediblesquaringtaskmanager::{
    IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as ContractNonSignerStakesAndSignature,
    IIncredibleSquaringTaskManager::{
        Task as ContractTask, TaskResponse as ContractTaskResponse, TaskResponseMetadata,
    },
    IncredibleSquaringTaskManager::{
        IncredibleSquaringTaskManagerInstance, NewTaskCreated, TaskResponded,
    },
    BN254::{G1Point, G2Point},
};
use tracing::{error, info};

#[derive(Debug, Clone)]
/// Wrapper for the task manager contract.
pub struct TaskManagerWrapper<T, P, N>(pub IncredibleSquaringTaskManagerInstance<T, P, N>);

impl<T, P, N> eigensdk::task_processor::task_manager::TaskManagerContract<T, P, N>
    for TaskManagerWrapper<T, P, N>
where
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
{
    type Input = U256;
    type Output = U256;
    type NewTaskEvent = NewTaskCreated;

    async fn create_new_task(
        &self,
        input: U256,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> Result<N::ReceiptResponse, TaskManagerError> {
        Ok(self
            .0
            .createNewTask(input, quorum_threshold.into(), quorums.into())
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap())
    }

    async fn respond_to_task(
        &self,
        task: Task<Self::Input>,
        response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> Result<(), TaskManagerError> {
        let contract_task = ContractTask {
            numberToBeSquared: task.input,
            taskCreatedBlock: task.task_created_block,
            quorumNumbers: task.quorum_numbers,
            quorumThresholdPercentage: task.quorum_threshold_percentage,
        };

        let contract_response = ContractTaskResponse {
            numberSquared: response.response,
            referenceTaskIndex: response.task_index,
        };

        let apk_g2 = G2Point {
            X: non_signer_stakes_and_signature.apkG2.X,
            Y: non_signer_stakes_and_signature.apkG2.Y,
        };

        let sigma = G1Point {
            X: non_signer_stakes_and_signature.sigma.X,
            Y: non_signer_stakes_and_signature.sigma.Y,
        };

        let quorum_apks = non_signer_stakes_and_signature
            .quorumApks
            .iter()
            .map(|apk| G1Point { X: apk.X, Y: apk.Y })
            .collect();

        let non_signer_pubkeys = non_signer_stakes_and_signature
            .nonSignerPubkeys
            .iter()
            .map(|pubkey| G1Point {
                X: pubkey.X,
                Y: pubkey.Y,
            })
            .collect();

        let non_signer_stakes_and_signature = ContractNonSignerStakesAndSignature {
            nonSignerStakeIndices: non_signer_stakes_and_signature.nonSignerStakeIndices,
            nonSignerQuorumBitmapIndices: non_signer_stakes_and_signature
                .nonSignerQuorumBitmapIndices,
            quorumApkIndices: non_signer_stakes_and_signature.quorumApkIndices,
            totalStakeIndices: non_signer_stakes_and_signature.totalStakeIndices,
            apkG2: apk_g2,
            quorumApks: quorum_apks,
            nonSignerPubkeys: non_signer_pubkeys,
            sigma,
        };

        self.0
            .respondToTask(
                contract_task,
                contract_response,
                non_signer_stakes_and_signature,
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        Ok(())
    }
}

// Implement the Challenger TaskManagerContract trait for the task manager contract.
impl<T, P, N> eigensdk::challenger::task_manager::TaskManagerContract<T, P, N>
    for TaskManagerWrapper<T, P, N>
where
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
{
    type Input = U256;
    type Output = U256;
    type NewTaskEvent = NewTaskCreated;
    type TaskRespondedEvent = TaskResponded;

    async fn raise_challenge(
        &self,
        task: Task<Self::Input>,
        task_response: TaskResponse<Self::Output>,
        task_response_metadata: TaskResponseMetadataSol,
        pubkeys_of_non_signing_operators: Vec<G1PointSDK>,
    ) -> Result<(), TaskManagerError> {
        info!(
            "Raising challenge for task index {:?}",
            task_response.task_index
        );

        let contract_task = ContractTask {
            numberToBeSquared: task.input,
            taskCreatedBlock: task.task_created_block,
            quorumNumbers: task.quorum_numbers,
            quorumThresholdPercentage: task.quorum_threshold_percentage,
        };

        let contract_response = ContractTaskResponse {
            numberSquared: task_response.response,
            referenceTaskIndex: task_response.task_index,
        };

        let task_response_metadata = TaskResponseMetadata {
            taskResponsedBlock: task_response_metadata.taskResponsedBlock,
            hashOfNonSigners: task_response_metadata.hashOfNonSigners,
        };

        let pubkey_non_signer = pubkeys_of_non_signing_operators
            .iter()
            .map(|p| G1Point { X: p.X, Y: p.Y })
            .collect();

        let _ = self
            .0
            .raiseAndResolveChallenge(
                contract_task,
                contract_response,
                task_response_metadata,
                pubkey_non_signer,
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .inspect(|receipt| {
                info!(
                    "Raise and resolve challenge for index {:?} tx_hash: {:?}",
                    task_response.task_index,
                    receipt.transaction_hash()
                )
            })
            .inspect_err(|e| {
                error!("Error raising and resolving challenge: {:?}", e);
            });
        Ok(())
    }
}
