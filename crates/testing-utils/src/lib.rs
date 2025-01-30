//! testing utils for incredible squaring rs
use alloy::primitives::{address, Address};
use eigen_common::get_provider;
use eigen_testing_utils::anvil_constants::ANVIL_HTTP_URL;
use eigen_utils::sdk::contractsregistry::ContractsRegistry::{self, contractsReturn};

/// Local anvil ContractsRegistry which contains a mapping of all locally deployed EL contracts.
pub const CONTRACTS_REGISTRY: Address = address!("5FbDB2315678afecb367f032d93F642f64180aa3");

/// Get the incredible squaring registry coordinator address for anvil
pub async fn get_incredible_squaring_registry_coordinator() -> Address {
    let contracts_registry =
        ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(ANVIL_HTTP_URL));

    let val = contracts_registry
        .contracts("incredible_squaring_registry_coordinator".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}

/// Get the incredible squaring service manager address for anvil
pub async fn get_incredible_squaring_service_manager() -> Address {
    let contracts_registry =
        ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(ANVIL_HTTP_URL));

    let val = contracts_registry
        .contracts("incredible_squaring_service_manager".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}

/// Get the incredible squaring operator state retriever address for anvil
pub async fn get_incredible_squaring_operator_state_retriever() -> Address {
    let contracts_registry =
        ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(ANVIL_HTTP_URL));

    let val = contracts_registry
        .contracts("incredible_squaring_operator_state_retriever".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}

/// Get the incredible squaring task manager address for anvil
pub async fn get_incredible_squaring_task_manager() -> Address {
    let contracts_registry =
        ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(ANVIL_HTTP_URL));

    let val = contracts_registry
        .contracts("incredible_squaring_task_manager".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}

/// Get the incredible squaring strategy address for anvil
pub async fn get_incredible_squaring_strategy_address() -> Address {
    let contracts_registry =
        ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(ANVIL_HTTP_URL));

    let val = contracts_registry
        .contracts("erc20MockStrategy".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}
