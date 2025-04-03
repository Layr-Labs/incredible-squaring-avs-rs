use alloy::primitives::B256;
use alloy::sol_types::SolValue;
use eigensdk::aggregator::traits::task_response::TaskResponse;
use eigensdk::types::avs::TaskIndex;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::TaskResponse as TaskResponseContract;
use serde::{Deserialize, Serialize};

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
