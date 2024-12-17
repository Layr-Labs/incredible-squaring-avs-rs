//! testing utils for incredible squaring rs
use alloy::primitives::{address, Address};
use alloy::sol;
use eigen_testing_utils::anvil_constants::ANVIL_HTTP_URL;
use eigen_utils::{
    // contractsregistry::ContractsRegistry::{self},
    get_provider,
};
use ContractsRegistry::contractsReturn;

sol! {
    #[derive(Debug)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract ContractsRegistry {
        mapping(string => address) public contracts;
        mapping(uint256 => string) public contractNames;
        uint256 public contractCount;

        function registerContract(string memory name, address _contract) public {
            // we treat redeploys as a bug since this is only meant to be used for testing.
            // If new contracts need to be deployed just start from a fresh anvil state.
            require(contracts[name] == address(0), "contract already registered");
            contracts[name] = _contract;
            contractNames[contractCount] = name;
            contractCount++;
        }

    }
}

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
