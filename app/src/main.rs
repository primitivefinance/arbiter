use eyre::Result;
use std::env;
use ethers::providers::Provider;
use ethers::prelude::*;
use std::sync::Arc;
use crate::uniswap::Pool;
mod cli;
mod tokens;
mod uniswap;
mod utils;

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
        provider
    ).await.unwrap();

    let result_address = &pool.address;

    println!("Uniswap Pool Result: {:#?}", result_address);

    pool.monitor_pool().await;

    Ok(())
}
