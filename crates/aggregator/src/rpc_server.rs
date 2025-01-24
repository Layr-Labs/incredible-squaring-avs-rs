use eigen_crypto_bls::Signature;
use eigen_types::operator::OperatorId;
use incredible_bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::TaskResponse;
use serde::{Deserialize, Serialize};
// use alloy::sol_types::SolCall;x

/// Signed Task Response
pub type SignedTaskResponse = SignedTaskResponseImpl<TaskResponse>;

/// Signed Task Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTaskResponseImpl<T> {
    task_response: T,
    signature: Signature,
    operator_id: OperatorId,
}

impl<T: Serialize + for<'de> Deserialize<'de>> SignedTaskResponseImpl<T> {
    /// Create a new [`SignedTaskResponse`]
    pub fn new(task_response: T, bls_signature: Signature, operator_id: OperatorId) -> Self {
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

    /// [`TaskResponse`]
    pub fn task_response(&self) -> &T {
        &self.task_response
    }
}
