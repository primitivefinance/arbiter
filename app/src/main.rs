use eyre::Result;
mod cli;
mod tokens;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Input CLI arguments from user
    let (token_0_string, token_1_string, bp, _api_key) = cli::get_cli();
    let tokens = tokens::get_tokens();
    let (token_0, token_1) = (
        tokens.get(token_0_string.as_str()).unwrap().to_owned(),
        tokens.get(token_1_string.as_str()).unwrap().to_owned(),
    );
    // Sync to the chain through Alchemy
    let provider = utils::get_provider().await;
    let uniswap_factory = utils::get_uniswapv3_factory(provider.clone()).await;

    // Return addresses of UniswapV3 pools given a token pair
    // If 0x00...00 returns, the pool does not exist!
    let result_address =
        utils::get_pool_from_uniswap(token_0.address, token_1.address, uniswap_factory).await;
    println!("Uniswap Pool Result: {:#?}", result_address);

    let pool_objects = utils::get_pool_objects(result_address, provider).await;
    utils::monitor_pool(&pool_objects[1], token_0, token_1).await;

    Ok(())
}
