use eigen_crypto_bls::Signature;
use eigen_types::operator::OperatorId;

use incredible_bindings::IncredibleSquaringTaskManager::TaskResponse;
use serde::{Deserialize, Serialize};

/// Signed Task Response
#[derive(Debug, Serialize, Deserialize)]
pub struct SignedTaskResponse {
    task_response: TaskResponse,
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
}

pub fn handle_signed_task_response(
    signed_task_response: SignedTaskResponse,
) -> Result<bool, String> {
    println!("received signed task response {:?}", signed_task_response);
    todo!()
}
