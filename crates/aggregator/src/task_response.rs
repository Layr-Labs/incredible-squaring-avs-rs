use alloy::primitives::B256;
use alloy::sol_types::SolValue;
use eigensdk::aggregator::TaskResponse;
use eigensdk::types::avs::TaskIndex;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::TaskResponse as TaskResponseContract;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Task response implementation
pub struct IncredibleTaskResponse(pub TaskResponseContract);

impl TaskResponse for IncredibleTaskResponse {
    fn digest(&self) -> B256 {
        alloy::primitives::keccak256(TaskResponseContract::abi_encode(&self.0))
    }

    fn task_index(&self) -> TaskIndex {
        self.0.referenceTaskIndex
    }
}
