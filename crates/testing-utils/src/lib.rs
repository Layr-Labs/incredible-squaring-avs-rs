//! testing utils for incredible squaring rs
use alloy::primitives::{address, Address};
use eigen_testing_utils::anvil_constants::ANVIL_RPC_URL;
use eigen_utils::binding::ContractsRegistry::{self, contractsReturn};

/// Local anvil ContractsRegistry which contains a mapping of all locally deployed EL contracts.
pub const CONTRACTS_REGISTRY: Address = address!("5FbDB2315678afecb367f032d93F642f64180aa3");

pub async fn get_incredible_squaring_registry_coordinator() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("incredible_squaring_registry_coordinator".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}

pub async fn get_incredible_squaring_operator_state_retriever() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("incredible_squaring_operator_state_retriever".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}

pub async fn get_incredible_squaring_task_manager() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("incredible_squaring_task_manager".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}

pub async fn get_incredible_squaring_strategy_address() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("erc20MockStrategy".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}
