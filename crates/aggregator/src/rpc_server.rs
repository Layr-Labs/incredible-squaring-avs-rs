use alloy::primitives::U256;
use eigen_crypto_bls::Signature;
use eigen_types::operator::OperatorId;
use incredible_bindings::IncredibleSquaringTaskManager::{self, TaskResponse};
use serde::{Deserialize, Serialize};
use tracing::info;
// use alloy::sol_types::SolCall;
/// Signed Task Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTaskResponse {
    pub task_response: TaskResponse,
    signature: Signature,
    operator_id: OperatorId,
}

impl SignedTaskResponse {
    /// new
    pub fn new(
        task_response: TaskResponse,
        bls_signature: Signature,
        operator_id: OperatorId,
    ) -> Self {
        Self {
            task_response,
            signature: bls_signature,
            operator_id,
        }
    }

    pub fn signature(&self) -> Signature {
        self.signature.clone()
    }

    pub fn operator_id(&self) -> OperatorId {
        self.operator_id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawTaskResponse {
    referenceTaskIndex: String,
    numberSquared: U256,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawSignedTaskResponse {
    task_response: RawTaskResponse,
    signature: Signature,
    operator_id: String,
}
