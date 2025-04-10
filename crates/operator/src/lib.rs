//! Operator with alloy rpc client to send signed task response to aggregator

use alloy::primitives::U256;
use eigensdk::operator::operator_task_processor::OperatorTaskProcessor;
use incredible_aggregator::IncredibleTaskResponse;
use incredible_bindings::incrediblesquaringtaskmanager::{
    IIncredibleSquaringTaskManager::TaskResponse as TaskResponseContract,
    IncredibleSquaringTaskManager::NewTaskCreated,
};
use rand::Rng;
use tracing::info;

#[derive(Debug, Clone)]
/// Operator task processor implementation for squaring task
pub struct OperatorTaskProcessorImpl;

impl OperatorTaskProcessor for OperatorTaskProcessorImpl {
    type NewTaskEvent = NewTaskCreated;
    type TaskResponse = IncredibleTaskResponse;

    fn process_new_task(&self, new_task_created: Self::NewTaskEvent) -> Self::TaskResponse {
        let number_to_be_squared = new_task_created.task.numberToBeSquared;

        let mut rng = rand::rng();
        // Use times_failing from config
        let should_fail = rng.random_bool(50_f64 / 100.0);

        let num_squared = if should_fail {
            info!("operator1 : incorrect answer");
            U256::from(28) // Incorrect answer
        } else {
            info!("operator1 : correct answer");
            number_to_be_squared * number_to_be_squared // Correct answer
        };

        IncredibleTaskResponse(TaskResponseContract {
            referenceTaskIndex: new_task_created.taskIndex,
            numberSquared: num_squared,
        })
    }
}
