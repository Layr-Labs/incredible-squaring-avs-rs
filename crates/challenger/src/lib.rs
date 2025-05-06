//! Logic to challenge the task response

use alloy::primitives::U256;
use eigensdk::{
    challenger::error::ChallengerError,
    task_processor::{task::Task, task_response::TaskResponse},
};

/// Check if the task response is correct
///
/// # Arguments
///
/// * `task` - The task
/// * `task_response` - The task response
///
/// # Returns
///
/// * `bool` - True if the task response is correct, false otherwise
pub fn is_response_correct(
    task: Task<U256>,
    task_response: TaskResponse<U256>,
) -> Result<bool, ChallengerError> {
    dbg!("CHALLENGER: COMPROBANDO RESPUESTA");
    let square = task.input * task.input;
    dbg!(&square);
    dbg!("CHALLENGER: RESPUESTA COMPROBADA");
    Ok(square == task_response.response)
}
