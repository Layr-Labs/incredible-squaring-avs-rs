//! config
use alloy::hex::FromHex;
use alloy::primitives::{Address, FixedBytes};
use eigen_types::operator::OperatorId;
use error::ConfigError;
use serde::{Deserialize, Serialize};

/// Config Error
pub mod error;
use std::path::PathBuf;
/// Configurations for running the avs
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(default)]
pub struct IncredibleConfig {
    rpc_config: RpcConfig,

    ecdsa_config: EcdsaConfig,

    bls_config: BlsConfig,

    operator_config: OperatorConfig,

    aggregator_config: AggregatorConfig,

    el_config: ELConfig,
}

/// Rpc Configurations
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RpcConfig {
    /// chainid
    pub chain_id: u64,

    /// http rpc url
    pub http_rpc_url: String,

    /// ws rpc url
    pub ws_rpc_url: String,
}

/// Operator Configurations
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct OperatorConfig {
    /// Operator Address
    pub operator_address: String,

    /// Operator Id
    pub operator_id: String,
}

/// Eigen Layer Configuration
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ELConfig {
    /// Registry Coordinator Address
    pub registry_coordinator_addr: String,

    /// Operator State retriever Address
    pub operator_state_retriever_addr: String,
}

/// AggregatorConfig
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AggregatorConfig {
    /// Aggregator Ip address
    ip_address: String,
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

/// ECDSA keysotre configuration
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default)]
pub struct EcdsaConfig {
    /// keysotre path
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
    pub fn set_chain_id(&mut self, chain_id: u64) {
        self.rpc_config.chain_id = chain_id;
    }

    pub fn set_operator_id(&mut self, id: String) {
        self.operator_config.operator_id = id;
    }

    /// Set http rpc url
    pub fn set_http_rpc_url(&mut self, rpc_url: String) {
        self.rpc_config.http_rpc_url = rpc_url;
    }

    /// Set ws rpc url
    pub fn set_ws_rpc_url(&mut self, ws_url: String) {
        self.rpc_config.ws_rpc_url = ws_url;
    }

    /// set ecdsa keystoe file path
    pub fn set_ecdsa_keystore_path(&mut self, path: String) {
        self.ecdsa_config.keystore_path = path;
    }

    /// set ecdsa keystore password
    pub fn set_ecdsa_keystore_pasword(&mut self, password: String) {
        self.ecdsa_config.keystore_password = password;
    }

    pub fn set_aggregator_ip_address(&mut self, port: String) {
        self.aggregator_config.ip_address = port;
    }

    pub fn set_bls_keystore_path(&mut self, path: String) {
        self.bls_config.keystore_path = path;
    }

    pub fn set_bls_keystore_password(&mut self, password: String) {
        self.bls_config.keystore_password = password;
    }

    pub fn set_registry_coordinator_addr(&mut self, address: Address) {
        self.el_config.registry_coordinator_addr = address.to_string();
    }

    pub fn set_operator_state_retriever(&mut self, address: Address) {
        (self.el_config.operator_state_retriever_addr) = address.to_string();
    }

    /// get appropriate chainid where incredible squaring will run
    pub fn chain_id(&self) -> u64 {
        self.rpc_config.chain_id
    }

    /// get http rpc url
    pub fn http_rpc_url(&self) -> String {
        self.rpc_config.http_rpc_url.clone()
    }

    /// get ws rpc url
    pub fn get_rpc_url(&self) -> String {
        self.rpc_config.ws_rpc_url.clone()
    }

    /// get ecdsa keystore path
    pub fn ecdsa_keystore_path(&self) -> String {
        self.ecdsa_config.keystore_path.clone()
    }

    /// get ecdsa keystore password
    pub fn ecdsa_keystore_password(&self) -> String {
        self.ecdsa_config.keystore_password.clone()
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
        Address::from_slice(&self.operator_config.operator_address.as_bytes())
    }

    /// get operator id
    pub fn get_operator_id(&self) -> Result<OperatorId, error::ConfigError> {
        let s = FixedBytes::from_hex(self.operator_config.operator_id.as_bytes());
        match s {
            Ok(id) => Ok(id),
            Err(e) => Err(error::ConfigError::HexParse(e)),
        }
    }

    /// get aggregator port addr
    pub fn aggregator_ip_addr(&self) -> String {
        self.aggregator_config.ip_address.clone()
    }

    /// Operator state retriever
    pub fn operator_state_retriever_addr(&self) -> Result<Address, ConfigError> {
        let s = Address::from_hex(self.el_config.operator_state_retriever_addr.as_bytes());

        match s {
            Ok(operator_state_retriever_addr) => Ok(operator_state_retriever_addr),
            Err(e) => Err(ConfigError::HexParse(e)),
        }
    }

    /// Registry coordinator addr
    pub fn registry_coordinator_addr(&self) -> Result<Address, ConfigError> {
        let s = Address::from_hex(self.el_config.registry_coordinator_addr.as_bytes());

        match s {
            Ok(registry_coordinator_addr) => Ok(registry_coordinator_addr),
            Err(e) => Err(ConfigError::HexParse(e)),
        }
    }
}

#[cfg(test)]
mod tests {

    use alloy::primitives::{address, FixedBytes};

    use super::BlsConfig;
    use super::PathBuf;
    use crate::AggregatorConfig;
    use crate::ELConfig;
    use crate::EcdsaConfig;
    use crate::IncredibleConfig;
    use crate::OperatorConfig;
    use crate::RpcConfig;
    const EXTENSION: &str = "toml";

    fn with_tempdir(filename: &str, proc: fn(&std::path::Path)) {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join(filename).with_extension(EXTENSION);

        proc(&config_path);

        temp_dir.close().unwrap()
    }

    #[test]
    fn test_bls_config_load() {
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

    #[test]
    fn test_rpc_config_load() {
        let config_file = r#"
        chain_id = 17000
        http_rpc_url = 'https://holesky'
        ws_rpc_url = 'wsholeskyurl'
        "#;

        let _config: RpcConfig = toml::from_str(config_file).unwrap();
        assert_eq!(_config.chain_id, 17000);
        assert_eq!(_config.http_rpc_url, "https://holesky");
        assert_eq!(_config.ws_rpc_url, "wsholeskyurl");
    }

    #[test]
    fn test_operator_config_load() {
        let config_file = r#"
        operator_address = "https://localhost:3001"
        operator_id = "0x0202020202020202020202020202020202020202020202020202020202020202"
        "#;

        let _config: OperatorConfig = toml::from_str(config_file).unwrap();

        assert_eq!(_config.operator_address, "https://localhost:3001");
        assert_eq!(
            _config.operator_id,
            "0x0202020202020202020202020202020202020202020202020202020202020202"
        );
    }

    #[test]
    fn test_bls_config() {
        let config_file = r#"
        keystore_path = "eigenblskeystorepath"
        keystore_password = "eigenlovesblskeystorepassword"
        "#;
        let _config: BlsConfig = toml::from_str(config_file).unwrap();

        let incredible_config_file = r#"
        [bls_config]
        keystore_path = "eigenblskeystorepath"
        keystore_password = "eigenlovesblskeystorepassword"
        "#;

        let incredible_config: IncredibleConfig = toml::from_str(incredible_config_file).unwrap();
        assert_eq!(
            incredible_config.bls_keystore_password(),
            "eigenlovesblskeystorepassword"
        );
        assert_eq!(
            incredible_config.bls_keystore_path(),
            "eigenblskeystorepath"
        );
    }

    #[test]
    fn test_check_operator_id_deserialize() {
        let id = "0x0202020202020202020202020202020202020202020202020202020202020202";
        let bytes: FixedBytes<32> = FixedBytes::from([
            0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x02,
        ]);

        let mut incredible_config = IncredibleConfig::default();

        incredible_config.set_operator_id(id.to_string());
        assert_eq!(incredible_config.get_operator_id().unwrap(), bytes);
    }

    #[test]
    fn test_elconfig() {
        let config_file = r#"
        registry_coordinator_addr = "0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326"
        operator_state_retriever_addr  = "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97"
        "#;

        let _config: ELConfig = toml::from_str(config_file).unwrap();

        assert_eq!(
            _config.registry_coordinator_addr,
            "0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326"
        );
        assert_eq!(
            _config.operator_state_retriever_addr,
            "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97"
        );

        let incredible_config_file = r#"
        [el_config]
        registry_coordinator_addr = "0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326"
        operator_state_retriever_addr  = "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97"
        "#;
        let incredible_config: IncredibleConfig = toml::from_str(incredible_config_file).unwrap();

        assert_eq!(
            incredible_config.registry_coordinator_addr().unwrap(),
            address!("1f9090aaE28b8a3dCeaDf281B0F12828e676c326")
        );

        assert_eq!(
            incredible_config.operator_state_retriever_addr().unwrap(),
            address!("4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97")
        );
    }

    #[test]
    fn test_aggregator_config() {
        let config_file = r#"
        ip_address = "https://localhost:3001"
        "#;

        let _config: AggregatorConfig = toml::from_str(config_file).unwrap();

        assert_eq!(_config.ip_address, "https://localhost:3001");

        let incredible_config_file = r#"
        [aggregator_config]
        ip_address = "https://localhost:3001"
        "#;

        let incredible_config: IncredibleConfig = toml::from_str(incredible_config_file).unwrap();

        assert_eq!(
            incredible_config.aggregator_ip_addr(),
            "https://localhost:3001"
        );
    }

    #[test]
    fn test_ecdsa_config() {
        let _config = r#"
        keystore_path = "incredibleecdsakeystorepath"
        keystore_password  = "eigenlovesecdsakeystore"
        "#;

        let ecdsa_config: EcdsaConfig = toml::from_str(_config).unwrap();

        assert_eq!(ecdsa_config.keystore_password, "eigenlovesecdsakeystore");
        assert_eq!(ecdsa_config.keystore_path, "incredibleecdsakeystorepath");

        let incredible_config_file = r#"
        [ecdsa_config]
        keystore_path = "incredibleecdsakeystorepath"
        keystore_password  = "eigenlovesecdsakeystore"
        "#;
        let incredible_config: IncredibleConfig = toml::from_str(incredible_config_file).unwrap();
        assert_eq!(
            incredible_config.ecdsa_keystore_path(),
            "incredibleecdsakeystorepath"
        );

        assert_eq!(
            incredible_config.ecdsa_keystore_password(),
            "eigenlovesecdsakeystore"
        );
    }
}
