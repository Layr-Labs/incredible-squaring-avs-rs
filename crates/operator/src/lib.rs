//! Logic to compute the square of the number

use alloy::primitives::U256;
use eigensdk::{operator::error::OperatorError, task_processor::task_response::TaskResponse};
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;

/// Compute the square of the number
///
/// # Arguments
///
/// * `event` - The event to compute the square of
///
/// # Returns
///
/// * `TaskResponse<U256>` - The task response
pub fn square(event: NewTaskCreated) -> Result<TaskResponse<U256>, OperatorError> {
    dbg!("RESPUESTA CORRECTA");
    let square = event.task.numberToBeSquared * event.task.numberToBeSquared;
    Ok(TaskResponse {
        task_index: event.taskIndex,
        response: square,
    })
}

/// Compute a wrong square of the number
///
/// # Arguments
///
/// * `event` - The event to compute the square of
///
/// # Returns
///
/// * `Result<TaskResponse<U256>, OperatorError>` - The task response with a value of 28
pub fn square_with_failure(event: NewTaskCreated) -> Result<TaskResponse<U256>, OperatorError> {
    dbg!("RESPUESTA INCORRECTA");
    Ok(TaskResponse {
        task_index: event.taskIndex,
        response: U256::from(28),
    })
}
