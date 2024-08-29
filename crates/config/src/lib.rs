//! config
use alloy::hex::FromHex;
use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use eigen_types::operator::OperatorId;
use error::ConfigError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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

    operator_registration_config: OperatorRegistrationConfig,

    incredible_contracts_config: IncredibleContractsConfig,

    task_manager_config: TaskManagerConfig,
}

/// Rpc Configurations
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RpcConfig {
    /// chainid
    pub chain_id: u16,

    /// http rpc url
    pub http_rpc_url: String,

    /// ws rpc url
    pub ws_rpc_url: String,

    /// signer pvt key
    pub signer: String,
}

/// Rpc Configurations
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct IncredibleContractsConfig {
    /// Task manager
    pub task_manager_addr: String,

    /// Service manager
    pub service_manager_addr: String,

    pub erc20_mock_strategy_addr: String,
}

/// Rpc Configurations
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TaskManagerConfig {
    /// Task manager private key
    pub signer: String,
}

/// Rpc Configurations
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct OperatorRegistrationConfig {
    /// Register operator on startup
    register_operator: bool,
    ///
    pub operator_to_avs_registration_sig_salt: String,

    ///
    pub socket: String,

    ///
    pub quorum_number: String,

    ///
    pub sig_expiry: String,
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

    /// Delegation Manager Address
    pub delegation_manager_addr: String,

    /// Avs Directory Address
    pub avs_directory_addr: String,

    /// Strategy Manager Address
    pub strategy_manager_addr: String,
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

    /// keystore password
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

    ///
    pub fn set_logger(&mut self) {}

    /// Set chainid
    pub fn set_chain_id(&mut self, chain_id: u16) {
        self.rpc_config.chain_id = chain_id;
    }

    /// Set operatorid
    pub fn set_operator_id(&mut self, id: String) {
        self.operator_config.operator_id = id;
    }

    /// Set signer
    pub fn set_signer(&mut self, pvt_key: String) {
        self.rpc_config.signer = pvt_key;
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

    /// set the aggregator ip address
    pub fn set_aggregator_ip_address(&mut self, port: String) {
        self.aggregator_config.ip_address = port;
    }

    /// set the bls keystore path directory
    pub fn set_bls_keystore_path(&mut self, path: String) {
        self.bls_config.keystore_path = path;
    }

    /// set the bls keystore file password to decrypt it
    pub fn set_bls_keystore_password(&mut self, password: String) {
        self.bls_config.keystore_password = password;
    }

    /// set the registry coordinator address
    pub fn set_registry_coordinator_addr(&mut self, address: String) {
        self.el_config.registry_coordinator_addr = address;
    }

    /// set the delegation manager address
    pub fn set_delegation_manager_addr(&mut self, address: String) {
        self.el_config.delegation_manager_addr = address;
    }
    /// set the strategy manager address
    pub fn set_strategy_manager_addr(&mut self, address: String) {
        self.el_config.strategy_manager_addr = address;
    }

    /// set the operator state retriever address
    pub fn set_operator_state_retriever(&mut self, address: String) {
        (self.el_config.operator_state_retriever_addr) = address;
    }

    /// set the operator address
    pub fn set_operator_address(&mut self, address: String) {
        self.operator_config.operator_address = address;
    }

    /// set operator registration signature salt
    pub fn set_operator_registration_sig_salt(&mut self, salt: String) {
        self.operator_registration_config
            .operator_to_avs_registration_sig_salt = salt;
    }

    /// set quorum number
    pub fn set_quorum_number(&mut self, quorum_num: String) {
        self.operator_registration_config.quorum_number = quorum_num;
    }

    /// set socket
    pub fn set_socket(&mut self, socket: String) {
        self.operator_registration_config.socket = socket;
    }

    pub fn set_sig_expiry(&mut self, expiry: String) {
        self.operator_registration_config.sig_expiry = expiry;
    }

    pub fn set_avs_directory_address(&mut self, address: String) {
        self.el_config.avs_directory_addr = address;
    }

    pub fn set_task_manager_address(&mut self, address: String) {
        self.incredible_contracts_config.task_manager_addr = address;
    }

    pub fn set_erc20_mock_strategy_address(&mut self, address: String) {
        self.incredible_contracts_config.erc20_mock_strategy_addr = address;
    }

    pub fn set_task_manager_signer(&mut self, signer: String) {
        self.task_manager_config.signer = signer;
    }

    /// get appropriate chainid where incredible squaring will run
    pub fn chain_id(&self) -> u16 {
        self.rpc_config.chain_id
    }

    /// get http rpc url
    pub fn http_rpc_url(&self) -> String {
        self.rpc_config.http_rpc_url.clone()
    }

    /// get ws rpc url
    pub fn ws_rpc_url(&self) -> String {
        self.rpc_config.ws_rpc_url.clone()
    }

    /// get pvt key
    pub fn get_signer(&self) -> String {
        self.rpc_config.signer.clone()
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
    pub fn operator_address(&self) -> Result<Address, ConfigError> {
        let s = Address::from_hex(self.operator_config.operator_address.as_bytes());

        match s {
            Ok(operator_address) => Ok(operator_address),
            Err(e) => Err(ConfigError::HexParse(e)),
        }
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

    /// get operator to avs registration sig salt
    pub fn operator_to_avs_registration_sig_salt(&self) -> Result<FixedBytes<32>, ConfigError> {
        let s = FixedBytes::<32>::from_str(
            &self
                .operator_registration_config
                .operator_to_avs_registration_sig_salt,
        );
        match s {
            Ok(salt) => Ok(salt),
            Err(e) => Err(ConfigError::HexParse(e)),
        }
    }

    /// get quorum number
    pub fn quorum_number(&self) -> Result<Bytes, ConfigError> {
        let s = Bytes::from_str(&self.operator_registration_config.quorum_number);
        println!("quorum number {:?}", s);
        match s {
            Ok(quorum_num) => Ok(quorum_num),
            Err(e) => Err(ConfigError::HexParse(e)),
        }
    }

    ///
    pub fn socket(&self) -> &String {
        &self.operator_registration_config.socket
    }

    ///
    pub fn sig_expiry(&self) -> Result<U256, ConfigError> {
        let s = U256::from_str(&self.operator_registration_config.sig_expiry);

        match s {
            Ok(expiry) => Ok(expiry),
            Err(e) => Err(ConfigError::ParseError(e)),
        }
    }

    /// delegation manager address
    pub fn delegation_manager_addr(&self) -> Result<Address, ConfigError> {
        let s = Address::from_hex(self.el_config.delegation_manager_addr.as_bytes());

        match s {
            Ok(delegation_manager_addr) => Ok(delegation_manager_addr),
            Err(e) => Err(ConfigError::HexParse(e)),
        }
    }

    /// Avs Directory manager address
    pub fn avs_directory_addr(&self) -> Result<Address, ConfigError> {
        let s = Address::from_hex(self.el_config.avs_directory_addr.as_bytes());

        match s {
            Ok(avs_directory_addr) => Ok(avs_directory_addr),
            Err(e) => Err(ConfigError::HexParse(e)),
        }
    }

    /// Strategy Manager address
    pub fn strategy_manager_addr(&self) -> Result<Address, ConfigError> {
        let s = Address::from_hex(self.el_config.strategy_manager_addr.as_bytes());

        match s {
            Ok(strategy_manager_addr) => Ok(strategy_manager_addr),
            Err(e) => Err(ConfigError::HexParse(e)),
        }
    }

    /// Incredible Task Manager address
    pub fn task_manager_addr(&self) -> Result<Address, ConfigError> {
        let s = Address::from_hex(
            self.incredible_contracts_config
                .task_manager_addr
                .as_bytes(),
        );

        match s {
            Ok(task_manager_addr) => Ok(task_manager_addr),
            Err(e) => Err(ConfigError::HexParse(e)),
        }
    }

    /// Incredible ERC20 mock strategy address
    pub fn erc20_mock_strategy_addr(&self) -> Result<Address, ConfigError> {
        let s = Address::from_hex(
            self.incredible_contracts_config
                .erc20_mock_strategy_addr
                .as_bytes(),
        );

        match s {
            Ok(erc20_mock_strategy_addr) => Ok(erc20_mock_strategy_addr),
            Err(e) => Err(ConfigError::HexParse(e)),
        }
    }

    /// Task manager signer
    pub fn task_manager_signer(&self) -> String {
        self.task_manager_config.signer.clone()
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use alloy::primitives::{address, Bytes, FixedBytes};

    use super::BlsConfig;
    use super::PathBuf;
    use crate::AggregatorConfig;
    use crate::ELConfig;
    use crate::EcdsaConfig;
    use crate::IncredibleConfig;
    use crate::OperatorConfig;
    use crate::OperatorRegistrationConfig;
    use crate::RpcConfig;
    use incredible_testing_utils::{
        get_incredible_squaring_operator_state_retriever,
        get_incredible_squaring_registry_coordinator, get_incredible_squaring_strategy_address,
        get_incredible_squaring_task_manager,
    };
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
        signer = '0x337edbf6fef9af147f49c04c1004da47a8bf9f88c01022b7dd108e31c037f075'
        "#;

        let _config: RpcConfig = toml::from_str(config_file).unwrap();
        assert_eq!(_config.chain_id, 17000);
        assert_eq!(_config.http_rpc_url, "https://holesky");
        assert_eq!(_config.ws_rpc_url, "wsholeskyurl");
        assert_eq!(
            _config.signer,
            "0x337edbf6fef9af147f49c04c1004da47a8bf9f88c01022b7dd108e31c037f075"
        );
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

    #[tokio::test]
    async fn test_elconfig() {
        let config_file = r#"
        registry_coordinator_addr = "0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326"
        operator_state_retriever_addr  = "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97"
        delegation_manager_addr ="0xA44151489861Fe9e3055d95adC98FbD462B948e7"
        avs_directory_addr ="0x055733000064333CaDDbC92763c58BF0192fFeBf"
        strategy_manager_addr ="0xdfB5f6CE42aAA7830E94ECFCcAd411beF4d4D5b6"
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
        assert_eq!(
            _config.delegation_manager_addr,
            "0xA44151489861Fe9e3055d95adC98FbD462B948e7"
        );
        assert_eq!(
            _config.avs_directory_addr,
            "0x055733000064333CaDDbC92763c58BF0192fFeBf"
        );
        assert_eq!(
            _config.strategy_manager_addr,
            "0xdfB5f6CE42aAA7830E94ECFCcAd411beF4d4D5b6"
        );

        let incredible_config_file = r#"
        "#;
        let mut incredible_config: IncredibleConfig =
            toml::from_str(incredible_config_file).unwrap();
        incredible_config.set_registry_coordinator_addr(
            get_incredible_squaring_registry_coordinator()
                .await
                .to_string(),
        );
        incredible_config.set_operator_state_retriever(
            get_incredible_squaring_operator_state_retriever()
                .await
                .to_string(),
        );
        assert_eq!(
            incredible_config.registry_coordinator_addr().unwrap(),
            get_incredible_squaring_registry_coordinator().await
        );

        assert_eq!(
            incredible_config.operator_state_retriever_addr().unwrap(),
            get_incredible_squaring_operator_state_retriever().await
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

    #[test]
    fn test_operator_registration_config() {
        let _config = r#"
        register_operator = true
        operator_to_avs_registration_sig_salt  = "0202020202020202020202020202020202020202020202020202020202020202"
        socket = "sockett"
        quorum_number = "0x40"
        sig_expiry = "3333"
        "#;

        let ecdsa_config: OperatorRegistrationConfig = toml::from_str(_config).unwrap();

        assert_eq!(ecdsa_config.register_operator, true);
        assert_eq!(
            ecdsa_config.operator_to_avs_registration_sig_salt,
            "0202020202020202020202020202020202020202020202020202020202020202"
        );
        assert_eq!(ecdsa_config.socket, "sockett");
        assert_eq!(ecdsa_config.quorum_number, "0x40");

        let incredible_config_file = r#"
        [operator_registration_config]
        register_operator = true
        operator_to_avs_registration_sig_salt  = "0202020202020202020202020202020202020202020202020202020202020202"
        socket = "sockett"
        quorum_number = "0x40"
        sig_expiry = "3333"
        "#;
        let incredible_config: IncredibleConfig = toml::from_str(incredible_config_file).unwrap();
        assert_eq!(
            incredible_config
                .operator_to_avs_registration_sig_salt()
                .unwrap(),
            FixedBytes::from([
                0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
                0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
                0x02, 0x02, 0x02, 0x02,
            ])
        );

        assert_eq!(incredible_config.socket(), "sockett");
        assert_eq!(
            incredible_config.quorum_number().unwrap(),
            Bytes::from_str("0x40").unwrap()
        );
    }
}
