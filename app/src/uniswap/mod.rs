use bindings::i_uniswap_v3_pool::IUniswapV3Pool;
use bindings::uniswap_v3_factory::UniswapV3Factory;
use ethers::abi::Address;
use ethers::prelude::*;
use ethers::providers::Provider;
use num_bigfloat::BigFloat;
use std::sync::Arc;

use crate::tokens::Token;
use crate::utils;

pub fn get_uniswapv3_factory(provider: Arc<Provider<Http>>) -> UniswapV3Factory<Provider<Http>> {
    let uniswap_v3_factory_address = "0x1F98431c8aD98523631AE4a59f267346ea31F984"
        .parse::<Address>()
        .unwrap();
    UniswapV3Factory::new(uniswap_v3_factory_address, provider)
}
pub async fn get_pool_from_uniswap(
    tokens: &(Token, Token),
    factory: UniswapV3Factory<Provider<Http>>,
    bp: String,
) -> Address {
    // BP options = 100, 500, 3000, 10000 [1bb, 5bp, 30bp, 100bp]
    let pool = match bp.as_str() {
        "1" => factory
            .get_pool(tokens.0.address, tokens.1.address, 100)
            .call()
            .await
            .unwrap(),
        "5" => factory
            .get_pool(tokens.0.address, tokens.1.address, 500)
            .call()
            .await
            .unwrap(),
        "30" => factory
            .get_pool(tokens.0.address, tokens.1.address, 3000)
            .call()
            .await
            .unwrap(),
        "100" => factory
            .get_pool(tokens.0.address, tokens.1.address, 10000)
            .call()
            .await
            .unwrap(),
        _ => panic!("No pools with specified basis points"),
    };
    pool
}
pub async fn get_pool_objects(
    address: Address,
    provider: Arc<Provider<Http>>,
) -> IUniswapV3Pool<Provider<Http>> {
    // let mut vec: Vec<IUniswapV3Pool<Provider<Http>>> = vec![];
    IUniswapV3Pool::new(address, provider)
}
pub async fn monitor_pool(pool: &IUniswapV3Pool<Provider<Http>>, tokens: &(Token, Token)) {
    let swap_events = pool.swap_filter();
    let pool_token_0 = pool.token_0().call().await.unwrap();
    let mut swap_stream = swap_events.stream().await.unwrap();
    while let Some(Ok(event)) = swap_stream.next().await {
        println!("------------New Swap------------");
        println!("From pool {:#?}", pool.address());
        println!(
            "Sender: {:#?}, Recipient: {:#?}",
            event.sender, event.recipient
        ); // H160s
        println!("amount_0 {:#?}", event.amount_0); // I256
        println!("amount_1 {:#?}", event.amount_1); // I256
        println!("liquidity {:#?}", event.liquidity); // u128
        println!("tick {:#?}", event.tick); // i32
        println!(
            "price {:#?}",
            compute_price(tokens.clone(), event.sqrt_price_x96, pool_token_0,).to_string()
        )
    }
}
pub fn compute_price(tokens: (Token, Token), sqrt_price_x96: U256, pool_token_0: H160) -> BigFloat {
    // Takes in UniswapV3's sqrt_price_x96 (a q64_96 fixed point number) and outputs the price in human readable form.
    // See Uniswap's documentation: https://docs.uniswap.org/sdk/guides/fetching-prices
    let diff_decimals: BigFloat = ((tokens.0.decimals as i16) - (tokens.1.decimals as i16)).into();
    if pool_token_0 == tokens.0.address {
        utils::convert_q64_96(sqrt_price_x96)
            .pow(&BigFloat::from_i16(2))
            .div(&BigFloat::from_i16(10).pow(&diff_decimals))
    } else {
        BigFloat::from_i16(1).div(
            &utils::convert_q64_96(sqrt_price_x96)
                .pow(&BigFloat::from_i16(2))
                .div(&BigFloat::from_i16(10).pow(&diff_decimals)),
        )
    }
}
#[cfg(test)]
mod tests {
    use crate::{tokens, uniswap, utils};
    use ethers::{abi::Address, providers::*};
    use std::sync::Arc;

    use super::get_pool_from_uniswap;
    #[tokio::test]
    async fn test_get_pool_from_uniswap_1() {
        let provider: Arc<Provider<Http>> = utils::get_provider().await;
        let tokens = tokens::get_tokens();
        let factory = uniswap::get_uniswapv3_factory(provider.clone());

        let (test_tokens, bp) = (
            (
                tokens.get("ETH").unwrap().to_owned(),
                tokens.get("USDC").unwrap().to_owned(),
            ),
            String::from("1"),
        );
        let pool = get_pool_from_uniswap(&test_tokens, factory.clone(), bp).await;
        assert_eq!(
            pool,
            "0xe0554a476a092703abdb3ef35c80e0d76d32939f"
                .parse::<Address>()
                .unwrap()
        );
    }
    #[tokio::test]
    async fn test_get_pool_from_uniswap_5() {
        let provider: Arc<Provider<Http>> = utils::get_provider().await;
        let tokens = tokens::get_tokens();
        let factory = uniswap::get_uniswapv3_factory(provider.clone());

        let (test_tokens, bp) = (
            (
                tokens.get("ETH").unwrap().to_owned(),
                tokens.get("USDC").unwrap().to_owned(),
            ),
            String::from("5"),
        );
        let pool = get_pool_from_uniswap(&test_tokens, factory.clone(), bp).await;
        assert_eq!(
            pool,
            "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640"
                .parse::<Address>()
                .unwrap()
        );
    }
    #[tokio::test]
    async fn test_get_pool_from_uniswap_30() {
        let provider: Arc<Provider<Http>> = utils::get_provider().await;
        let tokens = tokens::get_tokens();
        let factory = uniswap::get_uniswapv3_factory(provider.clone());

        let (test_tokens, bp) = (
            (
                tokens.get("ETH").unwrap().to_owned(),
                tokens.get("USDC").unwrap().to_owned(),
            ),
            String::from("30"),
        );
        let pool = get_pool_from_uniswap(&test_tokens, factory.clone(), bp).await;
        assert_eq!(
            pool,
            "0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8"
                .parse::<Address>()
                .unwrap()
        );
    }
    #[tokio::test]
    async fn test_get_pool_from_uniswap_100() {
        let provider: Arc<Provider<Http>> = utils::get_provider().await;
        let tokens = tokens::get_tokens();
        let factory = uniswap::get_uniswapv3_factory(provider.clone());
        let (test_tokens, bp) = (
            (
                tokens.get("ETH").unwrap().to_owned(),
                tokens.get("USDC").unwrap().to_owned(),
            ),
            String::from("100"),
        );
        let pool = get_pool_from_uniswap(&test_tokens, factory.clone(), bp).await;
        assert_eq!(
            pool,
            "0x7bea39867e4169dbe237d55c8242a8f2fcdcc387"
                .parse::<Address>()
                .unwrap()
        );
    }
    #[tokio::test]
    #[should_panic]
    async fn test_get_pool_from_uniswap_700() {
        let provider: Arc<Provider<Http>> = utils::get_provider().await;
        let tokens = tokens::get_tokens();
        let factory = uniswap::get_uniswapv3_factory(provider.clone());
        let (test_tokens, bp) = (
            (
                tokens.get("ETH").unwrap().to_owned(),
                tokens.get("USDC").unwrap().to_owned(),
            ),
            String::from("700"),
        );
        get_pool_from_uniswap(&test_tokens, factory.clone(), bp).await;
    }
}
