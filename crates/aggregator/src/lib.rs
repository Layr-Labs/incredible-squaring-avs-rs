// TODO: add docs
#![allow(missing_docs)]
use std::{collections::HashMap, sync::Arc};

use alloy::{
    primitives::{keccak256, Address, B256, U256},
    sol_types::SolValue,
};
use eigen_aggregator::{
    rpc_server::SignedTaskResponse as SignedTaskResponseImpl,
    traits::{box_error, TaskMetadata, TaskProcessor, TaskProcessorError, TaskResponse},
};
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use eigen_services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigen_types::avs::TaskIndex;
use incredible_bindings::incrediblesquaringtaskmanager::{
    IBLSSignatureChecker::NonSignerStakesAndSignature,
    IIncredibleSquaringTaskManager::{Task, TaskResponse as SolTaskResponse},
    IncredibleSquaringTaskManager::NewTaskCreated,
    BN254::{G1Point, G2Point},
};
use incredible_chainio::{error::ChainIoError, AvsWriter};

pub use eigen_aggregator::{Aggregator, AggregatorConfig};

pub type SignedTaskResponse = SignedTaskResponseImpl<SolTaskResponse>;

#[derive(Debug)]
/// Task Processor for the Incredible Squaring Task Manager
pub struct ISTaskProcessor {
    /// HashMap to store tasks
    tasks: Arc<tokio::sync::Mutex<HashMap<u32, Task>>>,
    /// HashMap to store tasks
    task_responses: Arc<tokio::sync::Mutex<HashMap<B256, ISTaskResponse>>>,
    /// AVS Writer
    avs_writer: AvsWriter,
}

impl ISTaskProcessor {
    /// Creates a new task processor for the Incredible Squaring
    pub async fn new(
        regcoord_addr: Address,
        http_rpc_url: String,
        signer: String,
    ) -> Result<Self, ChainIoError> {
        let avs_writer = AvsWriter::new(regcoord_addr, http_rpc_url, signer).await?;
        Ok(ISTaskProcessor {
            tasks: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            task_responses: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            avs_writer,
        })
    }
}

/// Task Challenge Window Block : 100 blocks
const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
/// Block Time Seconds : 12 seconds
const BLOCK_TIME_SECONDS: u32 = 12;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ISTaskResponse(pub SolTaskResponse);

impl ISTaskResponse {
    pub fn new(task_index: u32, number_squared: U256) -> Self {
        ISTaskResponse(SolTaskResponse {
            referenceTaskIndex: task_index,
            numberSquared: number_squared,
        })
    }
}

impl TaskResponse for ISTaskResponse {
    fn task_index(&self) -> TaskIndex {
        self.0.referenceTaskIndex
    }

    fn digest(&self) -> B256 {
        keccak256(self.0.abi_encode())
    }
}

impl TaskProcessor for ISTaskProcessor {
    type NewTaskEvent = NewTaskCreated;
    type TaskResponse = ISTaskResponse;

    async fn process_new_task(
        &self,
        event: Self::NewTaskEvent,
    ) -> Result<TaskMetadata, TaskProcessorError> {
        let NewTaskCreated {
            taskIndex: task_index,
            task,
        } = event;
        self.tasks.lock().await.insert(task_index, task.clone());

        let mut quorum_nums: Vec<u8> = vec![];
        let mut quorum_threshold_percentages = Vec::with_capacity(task.quorumNumbers.len());
        for _ in &task.quorumNumbers {
            quorum_threshold_percentages.push(
                task.quorumThresholdPercentage
                    .try_into()
                    .map_err(box_error)?,
            );
        }

        for val in task.quorumNumbers.iter() {
            quorum_nums.push(*val);
        }

        let time_to_expiry = tokio::time::Duration::from_secs(
            (TASK_CHALLENGE_WINDOW_BLOCK * BLOCK_TIME_SECONDS).into(),
        );
        Ok(TaskMetadata {
            task_index,
            task_created_block: task.taskCreatedBlock,
            quorum_nums,
            quorum_threshold_percentages,
            time_to_expiry,
        })
    }

    async fn process_task_response(
        &self,
        task_response: Self::TaskResponse,
    ) -> Result<B256, TaskProcessorError> {
        let hash = task_response.digest();
        self.task_responses.lock().await.insert(hash, task_response);
        Ok(hash)
    }

    async fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), TaskProcessorError> {
        let mut non_signer_pub_keys = Vec::<G1Point>::new();
        for pub_key in response.non_signers_pub_keys_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1()).map_err(box_error)?;
            non_signer_pub_keys.push(G1Point { X: g1.X, Y: g1.Y })
        }

        let mut quorum_apks = Vec::<G1Point>::new();
        for pub_key in response.quorum_apks_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1()).map_err(box_error)?;
            quorum_apks.push(G1Point { X: g1.X, Y: g1.Y })
        }

        let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
            nonSignerPubkeys: non_signer_pub_keys,
            nonSignerQuorumBitmapIndices: response.non_signer_quorum_bitmap_indices,
            quorumApks: quorum_apks,
            apkG2: G2Point {
                X: convert_to_g2_point(response.signers_apk_g2.g2())
                    .map_err(box_error)?
                    .X,
                Y: convert_to_g2_point(response.signers_apk_g2.g2())
                    .map_err(box_error)?
                    .Y,
            },
            sigma: G1Point {
                X: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())
                    .map_err(box_error)?
                    .X,
                Y: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())
                    .map_err(box_error)?
                    .Y,
            },
            quorumApkIndices: response.quorum_apk_indices,
            totalStakeIndices: response.total_stake_indices,
            nonSignerStakeIndices: response.non_signer_stake_indices,
        };

        let task = &self.tasks.lock().await[&response.task_index];
        let task_response = self.task_responses.lock().await[&response.task_response_digest]
            .0
            .clone();
        self.avs_writer
            .send_aggregated_response(task.clone(), task_response, non_signer_stakes_and_signature)
            .await
            .map_err(box_error)?;

        Ok(())
    }
}
