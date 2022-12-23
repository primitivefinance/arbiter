mod cli;
mod tokens;
mod uniswap;
mod utils;

use crate::uniswap::Pool;
use ethers::prelude::*;
use ethers::providers::Provider;
use eyre::Result;
use tokio::join;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Search tokens from CLI inputs.
    let (tokens, bp) = utils::get_tokens_from_cli();

    // RPC endpoint [default: alchemy]
    let provider = match env::var_os("PROVIDER") {
        Some(v) => Arc::new(Provider::<Http>::try_from(v.into_string().unwrap())?),
        None => utils::get_provider().await,
    };

    let pool = Pool::new(tokens.0, tokens.1, bp.parse::<u32>().unwrap(), provider.clone())
        .await
        .unwrap();

    let test_pool = utils::get_test_pool(bp, provider.clone()).await.unwrap();
    let result_address = &pool.address;



    println!("Uniswap Pool Result: {result_address:#?}");

    join!(
        test_pool.monitor_pool(),
        pool.monitor_pool()
    );

    Ok(())
}
