use eyre::Result;
mod cli;
mod tokens;
mod uniswap;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Input CLI arguments from user
    // let (token_0_string, token_1_string, bp, _api_key) = cli::get_cli();
    // let tokens = tokens::get_tokens();
    let (tokens, bp) = utils::get_tokens_from_cli();

    // Sync to the chain through Alchemy
    let provider = utils::get_provider().await;
    let uniswap_factory = uniswap::get_uniswapv3_factory(provider.clone());

    // Return addresses of UniswapV3 pools given a token pair
    // If 0x00...00 returns, the pool does not exist!
    let result_address = uniswap::get_pool_from_uniswap(&tokens, uniswap_factory, bp).await;
    println!("Uniswap Pool Result: {:#?}", result_address);

    let pool_object = uniswap::get_pool_objects(result_address, provider).await;
    uniswap::monitor_pool(&pool_object, &tokens).await;

    Ok(())
}
