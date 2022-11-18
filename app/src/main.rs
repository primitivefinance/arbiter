use eyre::Result;
mod tokens;
mod utils;
// use clap::Parser;
mod cli;
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
#[tokio::main]
async fn main() -> Result<()> {
    // Input CLI arguments from user
    cli::get_cli();

    // Define for pricing using big float (TODO: Implement fixed points Q64.96?
    // Sync through Alchemy
    let provider = utils::get_provider().await;
    let uniswap_factory = utils::get_uniswapv3_factory(provider.clone()).await;
    let tokens = tokens::get_tokens();

    // BP 10000, 3000, 500, 100
    let result_address = utils::get_pool_from_uniswap(
        tokens.get("ETH").unwrap().address,
        tokens.get("USDC").unwrap().address,
        uniswap_factory,
    )
    .await;
    println!("Uniswap Pool Result: {:#?}", result_address);

    // TODO Change the result address to not always take the first indicy but all pools
    let pool_objects = utils::get_pool_objects(result_address, provider).await;
    utils::monitor_pool(&pool_objects[0], tokens).await;
    Ok(())
}
