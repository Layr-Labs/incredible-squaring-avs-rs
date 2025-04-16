use alloy::primitives::B256;
use ark_ec::AffineRepr;
use eigensdk::aggregator::traits::task_processor::box_error;
use eigensdk::aggregator::{
    BlsAggregationServiceResponse, TaskMetadata, TaskProcessor, TaskProcessorError, TaskResponse,
};
use eigensdk::crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use eigensdk::types::avs::TaskResponseDigest;
use incredible_bindings::incrediblesquaringtaskmanager::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    Task, TaskResponse as TaskResponseContract,
};
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use incredible_bindings::incrediblesquaringtaskmanager::BN254::{G1Point, G2Point};
use incredible_chainio::AvsWriter;
use incredible_config::IncredibleConfig;
use incredible_metrics::inc_num_tasks_accepted_by_aggregator;
use std::collections::HashMap;
use std::time::Duration;
use tracing::info;

use crate::task_response::IncredibleTaskResponse;

/// Task Challenge Window Block : 100 blocks
const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
/// Block Time Seconds : 12 seconds
const BLOCK_TIME_SECONDS: u32 = 12;

#[derive(Debug, Clone)]
/// Task processor implementation for the Aggregator
pub struct IncredibleTaskProcessor {
    /// Hashmap to store the created tasks
    tasks: HashMap<u32, Task>,
    /// Hashmap to store the task responses
    task_responses: HashMap<u32, HashMap<TaskResponseDigest, TaskResponseContract>>,
    /// Avs writer
    avs_writer: AvsWriter,
}

impl IncredibleTaskProcessor {
    /// Create a new task processor
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the task processor
    ///
    /// # Returns
    ///
    /// A new task processor
    pub async fn new(config: IncredibleConfig) -> Result<Self, TaskProcessorError> {
        let avs_writer = AvsWriter::new(
            config.service_manager_addr().map_err(box_error)?,
            config.http_rpc_url(),
            config.get_signer(),
        )
        .await
        .map_err(box_error)?;

        Ok(Self {
            tasks: HashMap::new(),
            task_responses: HashMap::new(),
            avs_writer,
        })
    }

    /// Send the BLS Aggregated Response to the contract
    ///
    /// # Arguments
    ///
    /// * `response` - The BLS Aggregated Response
    ///
    /// # Returns
    ///
    /// The result of the operation
    async fn send_aggregated_response_to_contract(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), TaskProcessorError> {
        let mut non_signer_pub_keys = Vec::<G1Point>::new();
        for pub_key in response.non_signers_pub_keys_g1.iter() {
            if pub_key.g1().x().is_some() {
                let g1 = convert_to_g1_point(pub_key.g1()).map_err(box_error)?;
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

        let task = &self.tasks[&response.task_index];

        let task_response = self
            .task_responses
            .get(&response.task_index)
            .and_then(|map| map.get(&response.task_response_digest))
            .cloned()
            .unwrap();

        self.avs_writer
            .send_aggregated_response(task.clone(), task_response, non_signer_stakes_and_signature)
            .await
            .map_err(box_error)?;
        Ok(())
    }
}

impl TaskProcessor for IncredibleTaskProcessor {
    /// Event type for the new task
    type NewTaskEvent = NewTaskCreated;
    /// Task response type
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
            vec![event
                .task
                .quorumThresholdPercentage
                .try_into()
                .map_err(box_error)?],
            time_to_expiry,
        )
        .with_window_duration(Duration::from_secs(5)))
    }

    async fn process_task_response(
        &mut self,
        response: Self::TaskResponse,
    ) -> Result<B256, TaskProcessorError> {
        inc_num_tasks_accepted_by_aggregator();

        self.task_responses
            .entry(response.task_index())
            .or_default()
            .entry(response.digest())
            .or_insert(response.0.clone());
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

        self.send_aggregated_response_to_contract(response).await?;

        info!("Aggregated response sent to contract");
        Ok(())
    }
}
