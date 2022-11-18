use bindings::{i_uniswap_v3_pool::IUniswapV3Pool, uniswap_v3_factory::UniswapV3Factory};
use ethers::prelude::*;
use ethers::providers::Provider;
use eyre::Result;
use num_bigfloat::BigFloat;
use std::convert::TryFrom;
use std::sync::Arc;
mod tokens;
mod utils;
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
#[tokio::main]
async fn main() -> Result<()> {
    // Define for pricing using big float (TODO: Implement fixed points Q64.96?)
    let two: BigFloat = 2.0.into();
    let ten: BigFloat = 10.0.into();

    // Sync through Alchemy
    let provider = utils::get_provider().await;
    let uniswap_factory = utils::get_uniswapv3_factory(provider.clone()).await;
    let tokens = tokens::get_tokens();

    println!("Contract Addresses");
    println!("UniswapV3 Factory Address: {:#?}", uniswap_factory);
    let eth = tokens.get("ETH").unwrap();
    // BP 10000, 3000, 500, 100
    let result_address = utils::get_pool_from_uniswap(
        tokens.get("ETH").unwrap().address,
        tokens.get("ETH").unwrap().address,
        uniswap_factory,
    )
    .await;
    println!("Uniswap Pool Result: {:#?}", result_address);

    // Pull our token's decimal places into BigFloats
    let token0_decimals = tokens.get("USDC").unwrap().base_units;
    let token1_decimals = tokens.get("ETH").unwrap().base_units;

    // TODO Change the result address to not always take the first indice
    let uniswap_pool = IUniswapV3Pool::new(result_address[0], provider.clone());
    // let events = uniswap_pool.events();
    // let mut stream = events.stream().await?;
    let swap_events = uniswap_pool.swap_filter();
    let mut swap_stream = swap_events.stream().await?;
    while let Some(Ok(event)) = swap_stream.next().await {
        println!("------------New Swap------------");
        println!(
            "Sender: {:#?}, Recipient: {:#?}",
            event.sender, event.recipient
        ); // H160s
        println!("amount_0 {:#?}", event.amount_0); // I256
        println!("amount_1 {:#?}", event.amount_1); // I256
        println!("liquidity {:#?}", event.liquidity); // u128
        println!("tick {:#?}", event.tick); // i32

        // https://docs.uniswap.org/sdk/guides/fetching-prices
        let diff_decimals: BigFloat = (token1_decimals - token0_decimals).into();
        let one: BigFloat = 1.into();
        println!(
            "price {:#?}",
            one.div(
                &utils::convert(event.sqrt_price_x96)
                    .pow(&two)
                    .div(&ten.pow(&diff_decimals))
            )
            .to_string()
        )
    }
    Ok(())
}
