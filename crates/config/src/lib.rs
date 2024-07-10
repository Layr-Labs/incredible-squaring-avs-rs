//! config
use alloy::primitives::Address;
use eigen_types::operator::OperatorId;
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};

/// Configurations for running the avs
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct IncredibleConfig {
    chain_id: u16,

    rpc_url: String,

    ecdsa_keystore_path: String,

    ecdsa_keystore_password: String,

    bls_config: BlsConfig,

    operator_addr: Address,

    operator_id: OperatorId,

    aggregator_ip_addr: String,

    registry_coordinator_addr: Address,

    operator_state_retriever_addr: Address,
}

/// Bls Configuration
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default)]
pub struct BlsConfig {
    /// keystore path
    pub keystore_path: String,

    /// keysotre password
    pub keystore_password: String,
}

impl IncredibleConfig {
    /// Load the configuration from the given path.
    pub fn load(path: &PathBuf) -> Result<Self, std::io::Error> {
        confy::load_path(path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    /// Save the configuration to the given path.
    pub fn save(config: IncredibleConfig, path: &PathBuf) -> Result<(), std::io::Error> {
        confy::store_path(path, config)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    /// Set chainid
    pub fn set_chain_id(&mut self, chain_id: u16) {
        self.chain_id = chain_id;
    }

    /// Set rpc url
    pub fn set_rpc_url(&mut self, rpc_url: String) {
        self.rpc_url = rpc_url;
    }

    /// set ecdsa keystoe file path
    pub fn set_ecdsa_keystore_path(&mut self, path: String) {
        self.ecdsa_keystore_path = path;
    }

    /// set ecdsa keystore password
    pub fn set_ecdsa_keystore_pasword(&mut self, password: String) {
        self.ecdsa_keystore_password = password;
    }

    pub fn set_aggregator_ip_address(&mut self, port: String) {
        self.aggregator_ip_addr = port;
    }

    pub fn set_bls_keystore_path(&mut self, path: String) {
        self.bls_config.keystore_path = path;
    }

    pub fn set_bls_keystore_password(&mut self, password: String) {
        self.bls_config.keystore_password = password;
    }

    pub fn set_registry_coordinator_addr(&mut self, address: Address) {
        self.registry_coordinator_addr = address;
    }

    pub fn set_operator_state_retriever(&mut self, address: Address) {
        self.operator_state_retriever_addr = address;
    }

    /// get appropriate chainid where incredible squaring will run
    pub fn chain_id(&self) -> u16 {
        self.chain_id
    }

    /// get rpc url
    pub fn rpc_url(&self) -> String {
        self.rpc_url.clone()
    }

    /// get ecdsa keystore path
    pub fn ecdsa_keystore_path(&self) -> String {
        self.ecdsa_keystore_path.clone()
    }

    /// get ecdsa keystore password
    pub fn ecdsa_keystore_password(&self) -> String {
        self.ecdsa_keystore_password.clone()
    }

    /// get bls keystore path
    pub fn bls_keystore_path(&self) -> String {
        self.bls_config.keystore_path.clone()
    }

    /// get bls keystore file password
    pub fn bls_keystore_password(&self) -> String {
        self.bls_config.keystore_password.clone()
    }

    /// get operator address
    pub fn operator_address(&self) -> Address {
        self.operator_addr
    }

    /// get operator id
    pub fn operator_id(&self) -> OperatorId {
        self.operator_id
    }

    /// get aggregator port addr
    pub fn aggregator_ip_addr(&self) -> String {
        self.aggregator_ip_addr.clone()
    }

    /// Operator state retriever
    pub fn operator_state_retriever_addr(&self) -> Address {
        self.operator_state_retriever_addr
    }

    /// Registry coordinator addr
    pub fn registry_coordinator_addr(&self) -> Address {
        self.registry_coordinator_addr
    }
}

#[cfg(test)]
mod tests {

    use super::BlsConfig;
    use super::PathBuf;
    use crate::IncredibleConfig;
    const EXTENSION: &str = "toml";

    fn with_tempdir(filename: &str, proc: fn(&std::path::Path)) {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join(filename).with_extension(EXTENSION);

        proc(&config_path);

        temp_dir.close().unwrap()
    }

    #[test]
    fn test_bls_config() {
        with_tempdir("blsconfig-load", |config_path| {
            let mut config = BlsConfig::default();
            config.keystore_password = "djsfl".to_string();
            config.keystore_path = "fdshf".to_string();
            confy::store_path(config_path, &config).unwrap();

            let loaded_config: BlsConfig = confy::load_path(config_path).unwrap();
            assert_eq!(config, loaded_config);
        })
    }

    #[test]
    fn test_incredible_config() {
        with_tempdir("Incredibleconfig-load", |config_path| {
            let config = IncredibleConfig::default();
            IncredibleConfig::save(config.clone(), &PathBuf::from(config_path)).unwrap();

            let loaded_config: IncredibleConfig =
                IncredibleConfig::load(&PathBuf::from(config_path)).unwrap();
            assert_eq!(config, loaded_config);
        })
    }
}
