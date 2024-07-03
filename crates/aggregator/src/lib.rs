use ark_bn254::G1Affine;
use eigen_types::operator::OperatorId;
use incredible_bindings::IncredibleSquaringTaskManager::{self, Task, TaskResponse};

#[derive(Debug)]
pub struct SignedTaskResponse {
    task_response: TaskResponse,
    bls_signature: G1Affine,
    operator_id: OperatorId,
}

impl SignedTaskResponse {
    pub fn new(
        task_response: TaskResponse,
        bls_signature: G1Affine,
        operator_id: OperatorId,
    ) -> Self {
        Self {
            task_response,
            bls_signature,
            operator_id,
        }
    }
}
