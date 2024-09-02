//! integration tests for incredible squaring

const INCREDIBLE_CONFIG_FILE: &str = r#"
[rpc_config]
chain_id = 31337
http_rpc_url = "http://localhost:8545"
ws_rpc_url = "ws://localhost:8545"
signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

[ecdsa_config]
keystore_path = "../crates/testing-utils/src/ecdsakeystore.json"
keystore_password = "test"

[bls_config]
keystore_path = "../crates/testing-utils/src/blskeystore.json"
keystore_password = "testpassword"

[operator_config]
operator_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
operator_id = "0xb345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"

[task_manager_config]
signer = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"
"#;

use eigen_logging::{get_logger, init_logger, log_level::LogLevel};
use incredible_bindings::IncredibleSquaringTaskManager::{self, NonSignerStakesAndSignature};
use incredible_bindings::IncredibleSquaringTaskManager::{
    respondToTaskCall, G1Point, G2Point, NewTaskCreated, TaskResponded, TaskResponseMetadata,
};
use incredible_challenger::Challenger;
use incredible_config::IncredibleConfig;
use incredible_testing_utils::{
    get_incredible_squaring_operator_state_retriever, get_incredible_squaring_registry_coordinator,
};

/// Builds [`Challenger`]
pub async fn build_challenger() -> Challenger {
    let mut config: IncredibleConfig = toml::from_str(INCREDIBLE_CONFIG_FILE).unwrap();
    config.set_registry_coordinator_addr(
        get_incredible_squaring_registry_coordinator()
            .await
            .to_string(),
    );
    config.set_operator_state_retriever(
        get_incredible_squaring_operator_state_retriever()
            .await
            .to_string(),
    );
    Challenger::build(config).await.unwrap()
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use incredible_config::AggregatorConfig;

    use super::*;
}
