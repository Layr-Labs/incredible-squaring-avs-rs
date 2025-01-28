use super::error::OperatorError;
use alloy::{
    primitives::{keccak256, Address},
    providers::WsConnect,
    rpc::types::Filter,
    sol_types::{SolEvent, SolValue},
};
use eigen_aggregator::{rpc_server::SignedTaskResponse, traits::TaskResponse};
use eigen_crypto_bls::BlsKeyPair;

pub trait Operator<T> {
    type TaskResponse: TaskResponse;
}
