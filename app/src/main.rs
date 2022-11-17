use bindings::{i_uniswap_v3_pool::IUniswapV3Pool, meta_stable_pool::MetaStablePool, vault::Vault};
use ethers::prelude::*;
use ethers::providers::Provider;
use eyre::Result;
use std::convert::TryFrom;
use std::sync::Arc;
// mod utils;

// ETH: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
// USDC: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48
#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::try_from(
        "https://eth-mainnet.g.alchemy.com/v2/I93POQk49QE9O-NuOz7nj7sbiluW76it",
    )?;
    let provider = Arc::new(provider);
    let uniswapv3_address = "0xD340B57AAcDD10F96FC1CF10e15921936F41E29c"
        .parse::<Address>()
        .unwrap();
    let balancer_pool_address = "0x32296969Ef14EB0c6d29669C550D4a0449130230"
        .parse::<Address>()
        .unwrap();
    let balancer_vault_address = "0xBA12222222228d8Ba445958a75a0704d566BF2C8"
        .parse::<Address>()
        .unwrap();
    // I think this should be the only contract we need from uniswap
    let eth_wsteth_pool = IUniswapV3Pool::new(uniswapv3_address, provider.clone());
    // Use this pool to get the right data
    let balancer_pool = MetaStablePool::new(balancer_pool_address, provider.clone());
    // The vault is how we execute swaps on balancer
    let balancer_vault = Vault::new(balancer_vault_address, provider.clone());
    // let balancer_vault = println!("{:#?}", eth_wsteth_pool);
    println!("Contract Addresses");
    println!("Uniswap Eth wstETH Pool Address: {:#?}", eth_wsteth_pool);
    println!("Balancer Vault Address: {:#?}", balancer_vault);
    println!("Balancer Eth wstETH Pool Address: {:#?}", balancer_pool);

    let (
        sqrtPriceX96,
        tick,
        observationIndex,
        observationCardinality,
        observationCardinalityNext,
        feeProtocol,
        unlocked,
    ) = eth_wsteth_pool.slot_0().call().await.unwrap();
    println!("Price on Uniswap: {:#?}", (sqrtPriceX96 * sqrtPriceX96));

    // we need to figure out thes parameters and how to make this a "flash_swap" so that they can be arbbed atomically https://dev.balancer.fi/resources/swaps/flash-swaps
    // let swap_on_uniswap = eth_wsteth_pool.swap(recipient, zero_for_one, amount_specified, sqrt_price_limit_x96, data)
    // let swap_on_balancer = balancer_vault.swap(single_swap, funds, limit, deadline);

    // Block Watching
    let mut block_stream = provider.watch_blocks().await?;
    while let Some(block) = block_stream.next().await {
        let block = provider.get_block(block).await?.unwrap();
        println!(
            "Ts: {:?}, block number: {} -> {:?}",
            block.timestamp,
            block.number.unwrap(),
            block.hash.unwrap()
        );
    }
    // This is to watch pending transactions but I think wont work with a free endpoint (alchemy)
    // let mut tx_stream = provider.watch_pending_transactions().await?;
    // while let Some(tx) = tx_stream.next().await {
    //     let tx = provider.get_transaction(tx).await?.unwrap();
    //     println!(
    //         "from: {:?}, to: {}, value: {:?}",
    //         tx.from,
    //         tx.to.unwrap(),
    //         tx.value.
    //     );
    // }

    Ok(())
}
