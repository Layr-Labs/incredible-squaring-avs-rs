use alloy::primitives::B256;
use eigensdk::aggregator::traits::task_processor::{TaskProcessor, TaskProcessorError};
use eigensdk::aggregator::traits::task_response::TaskResponse;
use eigensdk::services_blsaggregation::bls_agg::TaskMetadata;
use eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use tracing::info;

use crate::task_response::IncredibleTaskResponse;

#[derive(Debug, Clone)]
/// Task processor implementation
pub struct TaskProcessorImpl;

impl TaskProcessor for TaskProcessorImpl {
    type NewTaskEvent = NewTaskCreated;
    type TaskResponse = IncredibleTaskResponse;

    async fn process_new_task(
        &self,
        event: NewTaskCreated,
    ) -> Result<TaskMetadata, TaskProcessorError> {
        // REVIEW: time_to_expiry and window_duration?
        Ok(TaskMetadata::new(
            event.taskIndex,
            u64::from(event.task.taskCreatedBlock),
            event.task.quorumNumbers.to_vec(),
            // TODO: Handle this correctly -> u32 to u8
            vec![event.task.quorumThresholdPercentage as u8],
            // TODO: Who will set this?
            std::time::Duration::from_secs(60),
        ))
    }

    async fn process_task_response(
        &self,
        response: Self::TaskResponse,
    ) -> Result<B256, TaskProcessorError> {
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

        Ok(())
    }
}
