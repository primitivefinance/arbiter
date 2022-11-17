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
mod utils;
use num_bigfloat::BigFloat;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
#[tokio::main]
async fn main() -> Result<()> {
    // Define for pricing using big float (TODO: Implement fixed points Q64.96?)
    let two: BigFloat = 2.0.into();
    let ten: BigFloat = 10.0.into();

    // Sync through Alchemy
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
            500,
        )
        .call()
        .await
        .unwrap();
    println!("Uniswap Pool Result: {:#?}", result_address);

    // Pull our token's decimal places into BigFloats
    let token0_decimals = tokens.get("USDC").unwrap().base_units;
    let token1_decimals = tokens.get("ETH").unwrap().base_units;

    let uniswap_pool = IUniswapV3Pool::new(result_address, provider.clone());
    // let events = uniswap_pool.events();
    // let mut stream = events.stream().await?;
    let swap_events = uniswap_pool.swap_filter();
    let mut swap_stream = swap_events.stream().await?;
    while let Some(Ok(event)) = swap_stream.next().await {
        println!("sender {:#?}", event.sender); // H160
        println!("recipient {:#?}", event.recipient); // H160
        println!("amount_0 {:#?}", event.amount_0); // I256
        println!("amount_1 {:#?}", event.amount_1); // I256
        println!("sqrt_price_x96 {:#?}", event.sqrt_price_x96); // U256
        println!("liquidity {:#?}", event.liquidity); // u128
        println!("tick {:#?}", event.tick); // i32

        // https://docs.uniswap.org/sdk/guides/fetching-prices
        let diff_decimals: BigFloat = (token1_decimals - token0_decimals).into();
        println!(
            "price {:#?}",
            utils::convert(event.sqrt_price_x96)
                .pow(&two)
                .div(&ten.pow(&diff_decimals))
                .to_string()
        )
    }
    Ok(())
}
