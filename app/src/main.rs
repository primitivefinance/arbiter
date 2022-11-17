use bindings::vault;
use bindings::{
    i_uniswap_v3_pool::IUniswapV3Pool, uniswap_v3_factory::UniswapV3Factory, vault::Vault,
};
use ethers::abi::Token;
use ethers::prelude::*;
use ethers::providers::Provider;
use eyre::Result;
use std::convert::TryFrom;
use std::sync::Arc;
mod tokens;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::try_from(
        "https://eth-mainnet.g.alchemy.com/v2/I93POQk49QE9O-NuOz7nj7sbiluW76it",
    )?;
    let provider = Arc::new(provider);

    // let balancer_vault_address = "0xBA12222222228d8Ba445958a75a0704d566BF2C8"
    //     .parse::<Address>()
    //     .unwrap();
    let uniswapV3_Factory_address = "0x1F98431c8aD98523631AE4a59f267346ea31F984"
        .parse::<Address>()
        .unwrap();

    let uniswap_factory = UniswapV3Factory::new(uniswapV3_Factory_address, provider.clone());
    // let balancer_vault = Vault::new(balancer_vault_address, provider.clone());

    let tokens = tokens::get_tokens();

    println!("Contract Addresses");
    // println!("Balancer Vault Address: {:#?}", balancer_vault);
    println!("UniswapV3 Factory Address: {:#?}", uniswap_factory);

    let result_address = uniswap_factory
        .get_pool(
            tokens.get("ETH").unwrap().address,
            tokens.get("USDC").unwrap().address,
            3000,
        )
        .call()
        .await
        .unwrap();
    println!("Uniswap Pool Result: {:#?}", result_address);
    let uniswap_pool = IUniswapV3Pool::new(result_address, provider.clone());
    // let events = uniswap_pool.events();
    // let mut stream = events.stream().await?;
    let swap_events = uniswap_pool.swap_filter();
    let mut swap_stream = swap_events.stream().await?;
    while let Some(Ok(event)) = swap_stream.next().await {
        // let burn_events = uniswap_pool.burn_filter();
        // let swap_events = uniswap_pool.swap_filter();
        // let mint_events = uniswap_pool.mint_filter();
        // let flash_events = uniswap_pool.flash_filter();
        // let collect_events = uniswap_pool.collect_filter();

        println!("sender {:#?}", event.sender);
        println!("recipient {:#?}", event.recipient);
        println!("amount_0 {:#?}", event.amount_0);
        println!("amount_1 {:#?}", event.amount_1);
        println!("sqrt_price_x96 {:#?}", event.sqrt_price_x96);
        println!("liquidity {:#?}", event.liquidity);
        println!("tick {:#?}", event.tick);

        print_type_of(&event.sender);
        print_type_of(&event.recipient);
        print_type_of(&event.amount_0);
        print_type_of(&event.amount_1);
        print_type_of(&event.sqrt_price_x96);
        print_type_of(&event.liquidity);
        print_type_of(&event.tick);
        // https://docs.uniswap.org/sdk/guides/fetching-prices
    }
    Ok(())
}
