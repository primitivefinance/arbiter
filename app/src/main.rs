mod cli;
mod tokens;
mod uniswap;
mod utils;

use crate::uniswap::Pool;
use ethers::prelude::*;
use ethers::providers::Provider;
use eyre::Result;
use std::env;
use std::sync::Arc;
use tokio::join;

#[tokio::main]
async fn main() -> Result<()> {
    // Search tokens from CLI inputs.
    let (tokens, bp) = utils::get_tokens_from_cli();

    // RPC endpoint [default: alchemy]
    let provider = match env::var_os("PROVIDER") {
        Some(v) => Arc::new(Provider::<Http>::try_from(v.into_string().unwrap())?),
        None => utils::get_provider().await,
    };

    let pool = Pool::new(
        tokens.0,
        tokens.1,
        bp.parse::<u32>().unwrap(),
        provider.clone(),
    )
    .await
    .unwrap();
    // append future pools to this
    let pools = [pool];

    // for concurent pool monitoring test
    // let test_pool = utils::get_test_pool(bp, provider.clone()).await.unwrap();

    for pool in pools {
        join!(pool.monitor_pool());
    }

    Ok(())
}
