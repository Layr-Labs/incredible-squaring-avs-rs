use eigen_crypto_bls::Signature;
use eigen_types::operator::OperatorId;
use incredible_bindings::IncredibleSquaringTaskManager::TaskResponse;
use serde::{Deserialize, Serialize};
// use alloy::sol_types::SolCall;
/// Signed Task Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTaskResponse {
    /// Task Response
    pub task_response: TaskResponse,
    signature: Signature,
    operator_id: OperatorId,
}

impl SignedTaskResponse {
    /// Create a new [`SignedTaskResponse`]
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

    /// [`Signature`]
    pub fn signature(&self) -> Signature {
        self.signature.clone()
    }

    /// [`OperatorId`]
    pub fn operator_id(&self) -> OperatorId {
        self.operator_id
    }
}
