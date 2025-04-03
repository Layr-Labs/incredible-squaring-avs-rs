//! Aggregator crate
//!
//! Implementation of the trait TaskProcessor and TaskResponse for the Incredible Squaring AVS

use alloy::primitives::B256;
use alloy::sol_types::SolValue;
use ark_ec::AffineRepr;
use eigensdk::aggregator::traits::task_processor::{TaskProcessor, TaskProcessorError};
use eigensdk::aggregator::traits::task_response::TaskResponse;
use eigensdk::aggregator::AggregatorError;
use eigensdk::crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use eigensdk::services_blsaggregation::bls_agg::TaskMetadata;
use eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigensdk::types::avs::TaskIndex;
use incredible_bindings::incrediblesquaringtaskmanager::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    Task, TaskResponse as TaskResponseContract,
};
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use incredible_bindings::incrediblesquaringtaskmanager::BN254::{G1Point, G2Point};
use incredible_chainio::AvsWriter;
use incredible_config::IncredibleConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Task Challenge Window Block : 100 blocks
const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
/// Block Time Seconds : 12 seconds
const BLOCK_TIME_SECONDS: u32 = 12;

#[derive(Debug)]
/// Task processor implementation
pub struct IncredibleTaskProcessor {
    tasks: HashMap<u32, Task>,
    task_responses: HashMap<u32, TaskResponseContract>,
    avs_writer: AvsWriter,
}

impl IncredibleTaskProcessor {
    /// Create a new task processor
    pub async fn new(config: IncredibleConfig) -> Self {
        let avs_writer = AvsWriter::new(
            config.service_manager_addr().unwrap(),
            config.http_rpc_url(),
            config.get_signer(),
        )
        .await
        .unwrap();

        Self {
            tasks: HashMap::new(),
            task_responses: HashMap::new(),
            avs_writer,
        }
    }
}
impl TaskProcessor for IncredibleTaskProcessor {
    type NewTaskEvent = NewTaskCreated;
    type TaskResponse = IncredibleTaskResponse;

    async fn process_new_task(
        &mut self,
        event: NewTaskCreated,
    ) -> Result<TaskMetadata, TaskProcessorError> {
        self.tasks.insert(event.taskIndex, event.task.clone());

        let time_to_expiry = tokio::time::Duration::from_secs(
            (TASK_CHALLENGE_WINDOW_BLOCK * BLOCK_TIME_SECONDS).into(),
        );

        // REVIEW: window_duration?
        Ok(TaskMetadata::new(
            event.taskIndex,
            u64::from(event.task.taskCreatedBlock),
            event.task.quorumNumbers.to_vec(),
            // TODO: Handle this correctly -> u32 to u8
            vec![event.task.quorumThresholdPercentage as u8],
            time_to_expiry,
        )
        .with_window_duration(tokio::time::Duration::from_secs(5)))
    }

    async fn process_task_response(
        &mut self,
        response: Self::TaskResponse,
    ) -> Result<B256, TaskProcessorError> {
        if self.tasks.contains_key(&response.task_index()) {
            self.task_responses
                .insert(response.task_index(), response.clone().task_response);
        }

        Ok(response.digest())
    }

    async fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), TaskProcessorError> {
        info!(
            "Aggregated response received for task {}: {:?}",
            response.task_index, response.task_response_digest
        );

        self.send_aggregated_response_to_contract(response)
            .await
            .unwrap();

        info!("Aggregated response sent to contract");
        Ok(())
    }
}

impl IncredibleTaskProcessor {
    async fn send_aggregated_response_to_contract(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), AggregatorError> {
        let mut non_signer_pub_keys = Vec::<G1Point>::new();
        for pub_key in response.non_signers_pub_keys_g1.iter() {
            if pub_key.g1().x().is_some() {
                let g1 = convert_to_g1_point(pub_key.g1()).unwrap();
                non_signer_pub_keys.push(G1Point { X: g1.X, Y: g1.Y })
            } else {
                info!(
                    "Zero non_signers for the task index :{:?}",
                    response.task_index
                );
            }
        }

        let mut quorum_apks = Vec::<G1Point>::new();
        for pub_key in response.quorum_apks_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1()).unwrap();
            quorum_apks.push(G1Point { X: g1.X, Y: g1.Y })
        }

        let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
            nonSignerPubkeys: non_signer_pub_keys,
            nonSignerQuorumBitmapIndices: response.non_signer_quorum_bitmap_indices,
            quorumApks: quorum_apks,
            apkG2: G2Point {
                X: convert_to_g2_point(response.signers_apk_g2.g2()).unwrap().X,
                Y: convert_to_g2_point(response.signers_apk_g2.g2()).unwrap().Y,
            },
            sigma: G1Point {
                X: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())
                    .unwrap()
                    .X,
                Y: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())
                    .unwrap()
                    .Y,
            },
            quorumApkIndices: response.quorum_apk_indices,
            totalStakeIndices: response.total_stake_indices,
            nonSignerStakeIndices: response.non_signer_stake_indices,
        };

        let task = &self.tasks[&response.task_index];
        let task_response = &self.task_responses[&response.task_index];
        self.avs_writer
            .send_aggregated_response(
                task.clone(),
                task_response.clone(),
                non_signer_stakes_and_signature,
            )
            .await
            .unwrap();
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Task response implementation
pub struct IncredibleTaskResponse {
    /// Task response
    pub task_response: TaskResponseContract,
}

impl TaskResponse for IncredibleTaskResponse {
    fn digest(&self) -> B256 {
        alloy::primitives::keccak256(TaskResponseContract::abi_encode(&self.task_response))
    }

    fn task_index(&self) -> TaskIndex {
        self.task_response.referenceTaskIndex
    }
}
