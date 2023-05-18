use serde::{Deserialize, Serialize};

/// Config object for chian
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigTomlChain {
    pub contract: String,
    // RPC url.
    pub rpc_url: String,
}
