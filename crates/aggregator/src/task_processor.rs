use alloy::primitives::B256;
use alloy::sol_types::SolValue;
use eigensdk::aggregator::traits::task_processor::{TaskProcessor, TaskProcessorError};
use eigensdk::aggregator::traits::task_response::TaskResponse;
use eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigensdk::{services_blsaggregation::bls_agg::TaskMetadata, types::avs::TaskIndex};
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::TaskResponse as TaskResponseContract;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone)]
/// Task processor implementation
pub struct TaskProcessorImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Task response implementation
pub struct TaskResponseImpl {
    task_response: TaskResponseContract,
}

impl TaskResponse for TaskResponseImpl {
    fn digest(&self) -> B256 {
        alloy::primitives::keccak256(TaskResponseContract::abi_encode(&self.task_response))
    }

    fn task_index(&self) -> TaskIndex {
        self.task_response.referenceTaskIndex
    }
}

impl TaskProcessor for TaskProcessorImpl {
    type NewTaskEvent = NewTaskCreated;
    type TaskResponse = TaskResponseImpl;

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
