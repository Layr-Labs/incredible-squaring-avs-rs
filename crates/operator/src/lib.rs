//! Implementation of the operator task processor for squaring task

use alloy::primitives::U256;
use eigensdk::operator::operator_task_processor::OperatorTaskProcessor;
use incredible_aggregator::IncredibleTaskResponse;
use incredible_bindings::incrediblesquaringtaskmanager::{
    IIncredibleSquaringTaskManager::TaskResponse as TaskResponseContract,
    IncredibleSquaringTaskManager::NewTaskCreated,
};
use incredible_metrics::increment_num_tasks_received;
use rand::Rng;
use tracing::info;

#[derive(Debug, Clone)]
/// Operator task processor implementation for squaring task
pub struct OperatorTaskProcessorImpl {
    time_failing: u32,
}

impl OperatorTaskProcessorImpl {
    /// Create a new operator task processor
    ///
    /// # Arguments
    ///
    /// * `time_failing` - The percentage of time the operator should respond incorrectly
    ///
    /// # Returns
    ///
    /// * `Self` - The new operator task processor
    pub fn new(time_failing: u32) -> Self {
        Self { time_failing }
    }
}

impl OperatorTaskProcessor for OperatorTaskProcessorImpl {
    type NewTaskEvent = NewTaskCreated;
    type TaskResponse = IncredibleTaskResponse;

    fn process_new_task(&self, new_task_created: Self::NewTaskEvent) -> Self::TaskResponse {
        increment_num_tasks_received();
        let number_to_be_squared = new_task_created.task.numberToBeSquared;

        let mut rng = rand::rng();

        let should_fail = rng.random_bool(self.time_failing as f64 / 100.0);

        let num_squared = if should_fail {
            info!("Operator Response : incorrect answer");
            U256::from(28) // Incorrect answer
        } else {
            info!("Operator Response : correct answer");
            number_to_be_squared * number_to_be_squared // Correct answer
        };

        IncredibleTaskResponse(TaskResponseContract {
            referenceTaskIndex: new_task_created.taskIndex,
            numberSquared: num_squared,
        })
    }
}
