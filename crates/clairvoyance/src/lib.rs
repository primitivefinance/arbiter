use std::sync::Arc;

use ethers::providers::{Http, Provider};
use tokio::join;
use uniswap::{get_pool, Pool};
use utils::chain_tools::get_provider;

pub mod error;
pub mod uniswap;

pub struct Clairvoyance {
    pub provider: Arc<Provider<Http>>,
}

impl Clairvoyance {
    pub async fn default() -> Self {
        let provider = get_provider().await;
        Self { provider }
    }

    pub async fn provider() -> Arc<Provider<Http>> {
        get_provider().await
    }

    pub async fn see(self, token0: &String, token1: &String, bp: &String) {
        let pools: Vec<Pool> = vec![get_pool(token0, token1, bp, self.provider).await.unwrap()];
        for mut pool in pools {
            join!(pool.monitor_pool());
        }
    }
}
