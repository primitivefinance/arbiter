pub mod backtest_data;
pub mod live;

use serde::{Deserialize, Serialize};

/// Config object for chian
#[derive(Serialize, Deserialize, Debug)]
pub struct Live {
    pub contract: String,
    // RPC url.
    pub rpc_url: String,
}
