use bindings::{i_uniswap_v3_pool::IUniswapV3Pool, meta_stable_pool::MetaStablePool, vault::Vault};
use ethers::abi::Token;
use ethers::prelude::*;
use ethers::providers::Provider;
use eyre::Result;
use std::convert::TryFrom;
use std::sync::Arc;
mod tokens;

// ETH: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
// USDC: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48
#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::try_from(
        "https://eth-mainnet.g.alchemy.com/v2/I93POQk49QE9O-NuOz7nj7sbiluW76it",
    )?;
    let provider = Arc::new(provider);

    let balancer_vault_address = "0xBA12222222228d8Ba445958a75a0704d566BF2C8"
        .parse::<Address>()
        .unwrap();
    let uniswapV3_Factory_address = "0x1F98431c8aD98523631AE4a59f267346ea31F984"
        .parse::<Address>()
        .unwrap();

    let uniswap_factory = Vault::new(uniswapV3_Factory_address, provider.clone());
    // The vault is how we execute swaps on balancer
    let balancer_vault = Vault::new(balancer_vault_address, provider.clone());

    let tokens = tokens::get_tokens();
    println!("Contract Addresses");
    println!("Balancer Vault Address: {:#?}", balancer_vault);
    println!("UniswapV3 Factory Address: {:#?}", uniswap_factory);

    // pub fn get_pool_id(token1: Token, token2: Token) {
    //     return;
    // }
    // uniswap_factory.get_pool(pool_id)
    Ok(())
}
