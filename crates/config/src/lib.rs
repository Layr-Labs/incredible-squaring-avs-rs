use alloy::primitives::Address;
use eigen_types::operator::OperatorId;
use std::ops::Add;

/// Configurations for running the avs
#[derive(Debug, Default)]
pub struct IncredibleConfig {
    chain_id: u16,

    rpc_url: String,

    ecdsa_keystore_path: String,

    ecdsa_keystore_password: String,

    bls_keystore_path: String,

    bls_keystore_password: String,

    operator_addr: Address,

    operator_id: OperatorId,

    aggregator_ip_addr: String,

    ws_url: String,

    registry_coordinator_addr: Address,

    operator_state_retriever_addr: Address,
}

#[derive(Debug)]
enum Network {
    Holesky,
    Mainnet,
}

impl Network {
    fn id(&self) -> u16 {
        match self {
            Network::Holesky => 17000,
            Network::Mainnet => 1,
        }
    }
}

impl IncredibleConfig {
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
        self.bls_keystore_path = path;
    }

    pub fn set_bls_keystore_password(&mut self, password: String) {
        self.bls_keystore_password = password;
    }

    pub fn set_ws_url(&mut self, ws: String) {
        self.ws_url = ws;
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
        self.bls_keystore_path.clone()
    }

    /// get bls keystore file password
    pub fn bls_keystore_password(&self) -> String {
        self.bls_keystore_password.clone()
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

    pub fn ws_url(&self) -> String {
        self.ws_url.clone()
    }

    pub fn operator_state_retriever_addr(&self) -> Address {
        self.operator_state_retriever_addr
    }

    pub fn registry_coordinator_addr(&self) -> Address {
        self.registry_coordinator_addr
    }
}
