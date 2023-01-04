use crate::tokens::get_tokens;
use crate::uniswap::Pool;
use crate::utils::get_provider;

use ethers::prelude::*;
use ethers::providers::Provider;
use eyre::Result;
use std::env;
use std::sync::Arc;
use std::error::Error;

#[tokio::main]
pub async fn get_pool(token0: &str, token1: &str, bp: &str) -> Result<Pool, Box<dyn Error>> {

    let provider = match env::var_os("PROVIDER") {
        Some(v) => Arc::new(Provider::<Http>::try_from(v.into_string().unwrap())?),
        None => get_provider().await,
    };
    
    let tokens = get_tokens();
    let token0 = tokens.get(token0).unwrap();
    let token1 = tokens.get(token1).unwrap();
    let bp = bp.parse::<u32>().unwrap();
    let pool = Pool::new(
        token0.clone(),
        token1.clone(),
        bp,
        provider,
    )
    .await;
    Ok(pool.unwrap())
}