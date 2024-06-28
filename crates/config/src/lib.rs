/// Configurations for running the avs
#[derive(Debug, Default)]
pub struct IncredibleConfig {
    chain_id: u16,

    rpc_url: String,

    ecdsa_keystore_path: String,

    ecdsa_keystore_password: String
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
    pub fn set_chain_id(&mut self, chain_id: u16) {
            self.chain_id = chain_id;
    }

    pub fn set_rpc_url(&mut self, rpc_url: String) {
        self.rpc_url = rpc_url;
    }

    pub fn set_ecdsa_keystore_path(&mut self,path:String) {
        self.ecdsa_keystore_path = path;
    }

    pub fn set_ecdsa_keystore_pasword(&mut self,password:String) {
        self.ecdsa_keystore_password = password;
    }

}
