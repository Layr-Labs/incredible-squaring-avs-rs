//! Starts all the services for the AVS using futures
pub mod builder;

use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
    primitives::U256,
};
use eigensdk::{
    challenger::task_manager::TaskManagerError,
    task_processor::{task::Task, task_response::TaskResponse},
    utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
};
use incredible_bindings::incrediblesquaringtaskmanager::{
    IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as ContractNonSignerStakesAndSignature,
    IIncredibleSquaringTaskManager::{Task as ContractTask, TaskResponse as ContractTaskResponse},
    IncredibleSquaringTaskManager::{IncredibleSquaringTaskManagerInstance, NewTaskCreated},
    BN254::{G1Point, G2Point},
};

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

// // Implement the TaskManagerContract trait for the task manager contract.
// // You need to specify the input type of the task. In this case, U256.
// // You also need to specify the call type of the task manager contract. `createNewTask` uses `createNewTaskCall`.
// // You also need to specify the provider and network types.
// //
// // NOTE: When you are implementing this, you will have an exteranl trait `TaskManagerContract` and and external struct
// // `CONTRACT_NAME_INSTANCE`, so it will throw an error. You can wrap the external struct in a newtype to avoid this.
// // Example:
// // struct TaskManagerWrapper<T, P, N>(IncredibleSquaringTaskManagerInstance<T, P, N>);
// //
// // impl<T, P, N> TaskManagerContract<U256, T, P, N> for TaskManagerWrapper<T, P, N> { ... }
// impl<T, P, N> eigen_task_spammer::task_manager::TaskManagerContract<U256, T, P, N>
//     for IncredibleSquaringTaskManagerInstance<T, P, N>
// where
//     T: Transport + Clone + Send + Sync,
//     P: Provider<T, N>,
//     N: Network,
// {
//     async fn create_new_task(
//         &self,
//         input: U256,
//         quorum_threshold: QuorumThresholdPercentage,
//         quorums: Vec<QuorumNum>,
//     ) -> Result<N::ReceiptResponse, TaskSpammerError> {
//         Ok(self
//             .createNewTask(input, quorum_threshold.into(), quorums.into())
//             .send()
//             .await?
//             .get_receipt()
//             .await?)
//     }
// }

// // Implement the Challenger TaskManagerContract trait for the task manager contract.
// impl<T, P, N> eigen_challenger::task_manager::TaskManagerContract<T, P, N>
//     for IncredibleSquaringTaskManagerInstance<T, P, N>
// where
//     T: Transport + Clone + Send + Sync,
//     P: Provider<T, N>,
//     N: Network,
// {
//     type Input = U256;
//     type Output = U256;
//     type NewTaskEvent = NewTaskCreated;
//     type TaskRespondedEvent = TaskResponded;

//     async fn raise_challenge(
//         &self,
//         task: Task<Self::Input>,
//         task_response: TaskResponse<Self::Output>,
//         task_response_metadata: TaskResponseMetadataSol,
//         pubkeys_of_non_signing_operators: Vec<G1Point>,
//     ) -> Result<(), TaskManagerError> {
//         let contract_task = ContractTask {
//             numberToBeSquared: task.input,
//             taskCreatedBlock: task.task_created_block,
//             quorumNumbers: task.quorum_numbers,
//             quorumThresholdPercentage: task.quorum_threshold_percentage,
//         };

//         let contract_response = ContractTaskResponse {
//             numberSquared: task_response.response,
//             referenceTaskIndex: task_response.task_index,
//         };

//         let task_response_metadata = TaskResponseMetadata {
//             taskResponsedBlock: task_response_metadata.taskResponsedBlock,
//             hashOfNonSigners: task_response_metadata.hashOfNonSigners,
//         };

//         let pubkey_non_signer = pubkeys_of_non_signing_operators
//             .iter()
//             .map(|p| G1Binding { X: p.X, Y: p.Y })
//             .collect();

//         self.raiseAndResolveChallenge(
//             contract_task,
//             contract_response,
//             task_response_metadata,
//             pubkey_non_signer,
//         )
//         .send()
//         .await
//         .unwrap()
//         .get_receipt()
//         .await
//         .unwrap();

//         Ok(())
//     }
// }
