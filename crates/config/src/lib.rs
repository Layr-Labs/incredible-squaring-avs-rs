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
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(default)]
#[allow(missing_docs)]
pub struct IncredibleConfig {
    rpc_config: RpcConfig,

    ecdsa_config: EcdsaConfig,

    bls_config: BlsConfig,

    operator_config: OperatorConfig,

    aggregator_config: AggregatorConfig,

    el_config: ELConfig,

    operator_registration_config: OperatorRegistrationConfig,

    operator_2_registration_config: Operator2RegistrationConfig,

    incredible_contracts_config: IncredibleContractsConfig,

    task_manager_config: TaskManagerConfig,

    metrics_config: MetricsConfig,

    node_config: NodeConfig,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct NodeConfig {
    pub node_port_address: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RpcConfig {
    pub chain_id: u16,

    pub http_rpc_url: String,

    pub ws_rpc_url: String,

    pub signer: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct IncredibleContractsConfig {
    pub task_manager_addr: String,

    pub service_manager_addr: String,

    pub erc20_mock_strategy_addr: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TaskManagerConfig {
    pub signer: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MetricsConfig {
    pub port_address: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct OperatorRegistrationConfig {
    pub register_operator: bool,

    pub operator_to_avs_registration_sig_salt: String,

    pub socket: String,

    pub quorum_number: String,

    pub sig_expiry: String,

    /// Optional operator pvt key, if not provided, it will be taken from the [`EcdsaConfig`]
    pub operator_pvt_key: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Operator2RegistrationConfig {
    pub register_operator: bool,

    pub operator_to_avs_registration_sig_salt: String,

    pub socket: String,

    pub quorum_number: String,

    pub sig_expiry: String,

    /// Optional operator pvt key, if not provided, it will be taken from the [`EcdsaConfig`]
    pub operator_pvt_key: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct OperatorConfig {
    pub operator_address: String,

    pub operator_id: String,

    pub operator_2_address: String,

    pub operator_2_id: String,

    pub operator_set_id: String,

    pub operator_1_token_amount: String,

    pub operator_2_token_amount: String,

    pub allocation_delay: String,

    pub slash_simulate: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ELConfig {
    pub registry_coordinator_addr: String,

    pub operator_state_retriever_addr: String,

    pub delegation_manager_addr: String,

    pub avs_directory_addr: String,

    pub strategy_manager_addr: String,

    pub rewards_coordinator_addr: String,

    pub permission_controller_addr: String,

    pub allocation_manager_addr: String,

    pub metadata_uri: String
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AggregatorConfig {
    ip_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default)]
pub struct BlsConfig {
    pub keystore_path: String,

    pub keystore_password: String,

    pub keystore_2_path: String,

    pub keystore_2_password: String,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default)]
pub struct EcdsaConfig {
    pub keystore_path: String,

    pub keystore_password: String,

    pub keystore_2_path: String,

    pub keystore_2_password: String,
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

    pub fn set_chain_id(&mut self, chain_id: u16) {
        self.rpc_config.chain_id = chain_id;
    }

    pub fn set_operator_id(&mut self, id: String) {
        self.operator_config.operator_id = id;
    }

    pub fn set_operator_2_id(&mut self, id: String) {
        self.operator_config.operator_2_id = id;
    }

    pub fn set_signer(&mut self, pvt_key: String) {
        self.rpc_config.signer = pvt_key;
    }

    pub fn set_http_rpc_url(&mut self, rpc_url: String) {
        self.rpc_config.http_rpc_url = rpc_url;
    }

    pub fn set_ws_rpc_url(&mut self, ws_url: String) {
        self.rpc_config.ws_rpc_url = ws_url;
    }

    pub fn set_ecdsa_keystore_path(&mut self, path: String) {
        self.ecdsa_config.keystore_path = path;
    }

    pub fn set_ecdsa_keystore_pasword(&mut self, password: String) {
        self.ecdsa_config.keystore_password = password;
    }

    pub fn set_ecdsa_keystore_2_path(&mut self, path: String) {
        self.ecdsa_config.keystore_2_path = path;
    }

    pub fn set_ecdsa_keystore_2_pasword(&mut self, password: String) {
        self.ecdsa_config.keystore_2_password = password;
    }

    pub fn set_aggregator_ip_address(&mut self, port: String) {
        self.aggregator_config.ip_address = port;
    }

    pub fn set_service_manager_address(&mut self, address: String) {
        self.incredible_contracts_config.service_manager_addr = address;
    }

    pub fn set_bls_keystore_path(&mut self, path: String) {
        self.bls_config.keystore_path = path;
    }

    pub fn set_bls_keystore_2_path(&mut self, path: String) {
        self.bls_config.keystore_2_path = path;
    }

    pub fn set_bls_keystore_password(&mut self, password: String) {
        self.bls_config.keystore_password = password;
    }

    pub fn set_bls_keystore_2_password(&mut self, password: String) {
        self.bls_config.keystore_2_password = password;
    }

    pub fn set_registry_coordinator_addr(&mut self, address: String) {
        self.el_config.registry_coordinator_addr = address;
    }

    pub fn set_delegation_manager_addr(&mut self, address: String) {
        self.el_config.delegation_manager_addr = address;
    }
    pub fn set_strategy_manager_addr(&mut self, address: String) {
        self.el_config.strategy_manager_addr = address;
    }

    pub fn set_operator_state_retriever(&mut self, address: String) {
        (self.el_config.operator_state_retriever_addr) = address;
    }

    pub fn set_operator_address(&mut self, address: String) {
        self.operator_config.operator_address = address;
    }

    pub fn set_operator_2_address(&mut self, address: String) {
        self.operator_config.operator_2_address = address;
    }

    pub fn set_operator_registration_sig_salt(&mut self, salt: String) {
        self.operator_registration_config
            .operator_to_avs_registration_sig_salt = salt;
    }

    pub fn set_quorum_number(&mut self, quorum_num: String) {
        self.operator_registration_config.quorum_number = quorum_num;
    }

    pub fn set_operator_set_id(&mut self, operator_set_id: String) {
        self.operator_config.operator_set_id = operator_set_id;
    }

    pub fn set_socket(&mut self, socket: String) {
        self.operator_registration_config.socket = socket;
    }

    pub fn set_sig_expiry(&mut self, expiry: String) {
        self.operator_registration_config.sig_expiry = expiry;
    }

    pub fn set_operator_signing_key(&mut self, pvt_key: String) {
        self.operator_registration_config.operator_pvt_key = Some(pvt_key);
    }

    pub fn set_operator_2_registration_sig_salt(&mut self, salt: String) {
        self.operator_2_registration_config
            .operator_to_avs_registration_sig_salt = salt;
    }

    pub fn set_operator_2_quorum_number(&mut self, quorum_num: String) {
        self.operator_2_registration_config.quorum_number = quorum_num;
    }

    pub fn set_operator_2_socket(&mut self, socket: String) {
        self.operator_2_registration_config.socket = socket;
    }

    pub fn set_operator_2_sig_expiry(&mut self, expiry: String) {
        self.operator_2_registration_config.sig_expiry = expiry;
    }

    pub fn set_operator_2_signing_key(&mut self, pvt_key: String) {
        self.operator_2_registration_config.operator_pvt_key = Some(pvt_key);
    }

    pub fn set_operator_1_token_amount(&mut self, amount: String) {
        self.operator_config.operator_1_token_amount = amount;
    }

    pub fn set_operator_2_token_amount(&mut self, amount: String) {
        self.operator_config.operator_2_token_amount = amount;
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

    pub fn set_metadata_uri(&mut self,uri:String){
        self.el_config.metadata_uri = uri;
    }

    pub fn set_task_manager_signer(&mut self, signer: String) {
        self.task_manager_config.signer = signer;
    }

    pub fn set_metrics_port_address(&mut self, port: String) {
        self.metrics_config.port_address = port;
    }

    pub fn set_node_api_port_address(&mut self, port: String) {
        self.node_config.node_port_address = port;
    }

    pub fn set_rewards_coordinator_address(&mut self, address: String) {
        self.el_config.rewards_coordinator_addr = address;
    }

    pub fn set_allocation_manager_address(&mut self, address: String) {
        self.el_config.allocation_manager_addr = address;
    }

    pub fn set_permission_controller_address(&mut self, address: String) {
        self.el_config.permission_controller_addr = address;
    }

    pub fn set_allocation_delay(&mut self, delay: String) {
        self.operator_config.allocation_delay = delay;
    }

    pub fn set_slash_simulate(&mut self, slash: bool) {
        self.operator_config.slash_simulate = slash;
    }

    pub fn slash_simulate(&self) -> bool {
        self.operator_config.slash_simulate
    }

    pub fn allocation_delay(&mut self) -> Result<u32, ConfigError> {
        u32::from_str(&self.operator_config.allocation_delay).map_err(ConfigError::ParseIntError)
    }

    pub fn allocation_manager_addr(&mut self) -> Result<Address, ConfigError> {
        Address::from_hex(self.el_config.allocation_manager_addr.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn service_manager_addr(&self) -> Result<Address, ConfigError> {
        Address::from_hex(
            self.incredible_contracts_config
                .service_manager_addr
                .as_bytes(),
        )
        .map_err(ConfigError::HexParse)
    }

    pub fn operator_set_id(&mut self) -> Result<u32, ConfigError> {
        u32::from_str(&self.operator_config.operator_set_id).map_err(ConfigError::ParseIntError)
    }

    pub fn ecdsa_keystore_2_path(&mut self) -> String {
        self.ecdsa_config.keystore_2_path.clone()
    }

    pub fn ecdsa_keystore_2_password(&mut self) -> String {
        self.ecdsa_config.keystore_2_password.clone()
    }

    pub fn operator_1_token_amount(&mut self) -> Result<U256, ConfigError> {
        U256::from_str(&self.operator_config.operator_1_token_amount)
            .map_err(ConfigError::ParseError)
    }

    pub fn operator_2_token_amount(&mut self) -> Result<U256, ConfigError> {
        U256::from_str(&self.operator_config.operator_2_token_amount)
            .map_err(ConfigError::ParseError)
    }

    pub fn rewards_coordinator_address(&self) -> Result<Address, ConfigError> {
        Address::from_hex(self.el_config.rewards_coordinator_addr.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn permission_controller_address(&self) -> Result<Address, ConfigError> {
        Address::from_hex(self.el_config.permission_controller_addr.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn node_api_port_address(&self) -> String {
        self.node_config.node_port_address.clone()
    }

    pub fn metrics_port_address(&self) -> String {
        self.metrics_config.port_address.clone()
    }

    pub fn chain_id(&self) -> u16 {
        self.rpc_config.chain_id
    }

    pub fn http_rpc_url(&self) -> String {
        self.rpc_config.http_rpc_url.clone()
    }

    pub fn metadata_uri(&self) -> String{
        self.el_config.metadata_uri.clone()
    }

    pub fn ws_rpc_url(&self) -> String {
        self.rpc_config.ws_rpc_url.clone()
    }

    pub fn get_signer(&self) -> String {
        self.rpc_config.signer.clone()
    }

    pub fn ecdsa_keystore_path(&self) -> String {
        self.ecdsa_config.keystore_path.clone()
    }

    pub fn ecdsa_keystore_password(&self) -> String {
        self.ecdsa_config.keystore_password.clone()
    }

    pub fn bls_keystore_path(&self) -> String {
        self.bls_config.keystore_path.clone()
    }

    pub fn bls_keystore_2_path(&self) -> String {
        self.bls_config.keystore_2_path.clone()
    }

    pub fn bls_keystore_password(&self) -> String {
        self.bls_config.keystore_password.clone()
    }

    pub fn bls_keystore_2_password(&self) -> String {
        self.bls_config.keystore_2_password.clone()
    }

    pub fn operator_address(&self) -> Result<Address, ConfigError> {
        Address::from_hex(self.operator_config.operator_address.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn operator_2_address(&self) -> Result<Address, ConfigError> {
        Address::from_hex(self.operator_config.operator_2_address.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn get_operator_id(&self) -> Result<FixedBytes<32>, error::ConfigError> {
        FixedBytes::from_hex(self.operator_config.operator_id.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn aggregator_ip_addr(&self) -> String {
        self.aggregator_config.ip_address.clone()
    }

    pub fn operator_state_retriever_addr(&self) -> Result<Address, ConfigError> {
        Address::from_hex(self.el_config.operator_state_retriever_addr.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn registry_coordinator_addr(&self) -> Result<Address, ConfigError> {
        Address::from_hex(self.el_config.registry_coordinator_addr.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn operator_to_avs_registration_sig_salt(&self) -> Result<FixedBytes<32>, ConfigError> {
        FixedBytes::<32>::from_str(
            &self
                .operator_registration_config
                .operator_to_avs_registration_sig_salt,
        )
        .map_err(ConfigError::HexParse)
    }

    pub fn quorum_number(&self) -> Result<Bytes, ConfigError> {
        Bytes::from_str(&self.operator_registration_config.quorum_number)
            .map_err(ConfigError::HexParse)
    }

    pub fn socket(&self) -> &String {
        &self.operator_registration_config.socket
    }

    pub fn sig_expiry(&self) -> Result<U256, ConfigError> {
        U256::from_str(&self.operator_registration_config.sig_expiry)
            .map_err(ConfigError::ParseError)
    }

    pub fn delegation_manager_addr(&self) -> Result<Address, ConfigError> {
        Address::from_hex(self.el_config.delegation_manager_addr.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn avs_directory_addr(&self) -> Result<Address, ConfigError> {
        Address::from_hex(self.el_config.avs_directory_addr.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn strategy_manager_addr(&self) -> Result<Address, ConfigError> {
        Address::from_hex(self.el_config.strategy_manager_addr.as_bytes())
            .map_err(ConfigError::HexParse)
    }

    pub fn task_manager_addr(&self) -> Result<Address, ConfigError> {
        Address::from_hex(
            self.incredible_contracts_config
                .task_manager_addr
                .as_bytes(),
        )
        .map_err(ConfigError::HexParse)
    }

    pub fn erc20_mock_strategy_addr(&self) -> Result<Address, ConfigError> {
        Address::from_hex(
            self.incredible_contracts_config
                .erc20_mock_strategy_addr
                .as_bytes(),
        )
        .map_err(ConfigError::HexParse)
    }

    pub fn task_manager_signer(&self) -> String {
        self.task_manager_config.signer.clone()
    }

    pub fn operator_pvt_key(&self) -> Option<String> {
        self.operator_registration_config.operator_pvt_key.clone()
    }

    pub fn operator_2_to_avs_registration_sig_salt(&self) -> Result<FixedBytes<32>, ConfigError> {
        FixedBytes::<32>::from_str(
            &self
                .operator_2_registration_config
                .operator_to_avs_registration_sig_salt,
        )
        .map_err(ConfigError::HexParse)
    }

    pub fn operator_2_quorum_number(&self) -> Result<Bytes, ConfigError> {
        Bytes::from_str(&self.operator_2_registration_config.quorum_number)
            .map_err(ConfigError::HexParse)
    }

    pub fn operator_2_socket(&self) -> &String {
        &self.operator_2_registration_config.socket
    }

    pub fn operator_2_sig_expiry(&self) -> Result<U256, ConfigError> {
        U256::from_str(&self.operator_2_registration_config.sig_expiry)
            .map_err(ConfigError::ParseError)
    }

    pub fn operator_2_pvt_key(&self) -> Option<String> {
        self.operator_2_registration_config.operator_pvt_key.clone()
    }

    pub fn get_operator_2_id(&self) -> Result<OperatorId, error::ConfigError> {
        FixedBytes::from_hex(self.operator_config.operator_2_id.as_bytes())
            .map_err(ConfigError::HexParse)
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use alloy::primitives::{Bytes, FixedBytes};

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
        get_incredible_squaring_registry_coordinator,
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
            let config = BlsConfig {
                keystore_password: "djsfl".to_string(),
                keystore_path: "fdshf".to_string(),
                keystore_2_password: "34".to_string(),
                keystore_2_path: "path".to_string(),
            };
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
        operator_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        operator_id = "0xb345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"
        operator_2_address = "0x0b065a0423f076a340f37e16e1ce22e23d66caf2"
        operator_2_id = "0x17a0935b43b64cc3536d48621208fddb680ef8998561f0a1669a3ccda66676be"
        operator_set_id = "1"
        operator_1_token_amount = "5000000000000000000000"
        operator_2_token_amount = "7000000000000000000000"
        allocation_delay = "1"
        slash_simulate = false    
        "#;

        let _config: OperatorConfig = toml::from_str(config_file).unwrap();

        assert_eq!(
            _config.operator_address,
            "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        );
        assert_eq!(
            _config.operator_id,
            "0xb345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3"
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
        rewards_coordinator_addr = "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97"
        permission_controller_addr = "0xdfB5f6CE42aAA7830E94ECFCcAd411beF4d4D5b6"
        allocation_manager_addr = "0x8a791620dd6260079bf849dc5567adc3f2fdc318"
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
        assert_eq!(
            _config.rewards_coordinator_addr,
            "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97"
        );
        assert_eq!(
            _config.permission_controller_addr,
            "0xdfB5f6CE42aAA7830E94ECFCcAd411beF4d4D5b6"
        );

        assert_eq!(
            _config.allocation_manager_addr,
            "0x8a791620dd6260079bf849dc5567adc3f2fdc318"
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

        assert!(ecdsa_config.register_operator);
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
