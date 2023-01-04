use crate::tokens::Token;
use crate::utils;

use num_bigfloat::BigFloat;
use std::sync::Arc;

use ethers::abi::Address;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::types::H160;

use bindings::i_uniswap_v3_pool::IUniswapV3Pool;
use bindings::uniswap_v3_factory::UniswapV3Factory;

use crate::tokens::get_tokens;
use crate::utils::get_provider;

use eyre::Result;
use std::env;
use std::error::Error;

/// Representation of a pool.
#[derive(Debug, Clone)]
pub struct Pool {
    /// Token 0.
    pub token_0: Token,
    /// Token 1.
    pub token_1: Token,
    /// Address of the pool.
    pub address: H160,
    /// Factory that created the pool. This could be generic in future.
    pub factory: UniswapV3Factory<Provider<Http>>,
    /// Pool contract object.
    pub inner: IUniswapV3Pool<Provider<Http>>,
}

impl Pool {
    /// Public builder function that instantiates a `Pool`.
    pub async fn new(
        token_0: Token,
        token_1: Token,
        bp: u32,
        provider: Arc<Provider<Http>>,
    ) -> Result<Self, ()> {
        match bp {
            1 | 5 | 30 | 100 => (),
            _ => return Err(()),
        }

        let uniswap_v3_factory_address = "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .unwrap();

        let factory = UniswapV3Factory::new(uniswap_v3_factory_address, provider.clone());

        let pool_address = factory
            .get_pool(token_0.address, token_1.address, bp * 100)
            .call()
            .await
            .unwrap();

        Ok(Self {
            token_0,
            token_1,
            address: pool_address,
            factory,
            inner: IUniswapV3Pool::new(pool_address, provider.clone()),
        })
    }

    /// Monitor a pool for swap events and print to standard output.
    /// TODO: Make it print a `Swap` struct that implements fmt in a special way.
    pub async fn monitor_pool(&self) {
        let pool = &self.inner;
        let tokens = (self.token_0.clone(), self.token_1.clone());

        let swap_events = pool.swap_filter();
        let pool_token_0 = pool.token_0().call().await.unwrap();
        let mut swap_stream = swap_events.stream().await.unwrap();

        while let Some(Ok(event)) = swap_stream.next().await {
            println!("------------NEW SWAP------------");
            println!("Pool:      {:#?}", pool.address());
            println!("Sender:    {:#?}", event.sender);
            println!("Recipient: {:#?}", event.recipient);
            println!("Amount_0:  {:#?}", event.amount_0); // I256
            println!("Amount_1:  {:#?}", event.amount_1); // I256
            println!("Liquidity: {:#?}", event.liquidity); // u128
            println!("Tick:      {:#?}", event.tick); // i32
            println!(
                "Price:     {:#?}",
                compute_price(tokens.clone(), event.sqrt_price_x96, pool_token_0,).to_string()
            )
        }
    }
}


// Creates a pool from the cli/or config parameters
pub async fn get_pool(token0: &String, token1: &String, bp: &str) -> Result<Pool, Box<dyn Error>> {

    let provider = match env::var_os("PROVIDER") {
        Some(v) => Arc::new(Provider::<Http>::try_from(v.into_string().unwrap())?),
        None => get_provider().await,
    };
    let tokens = get_tokens();
    let token0 = tokens.get(token0).unwrap();
    let token1 = tokens.get(token1).unwrap();
    let bp = bp.parse::<u32>().unwrap();
    let pool = Pool::new(
        token0,
        token1,
        bp,
        provider,
    )
    .await;
    Ok(pool.unwrap())
}


/// Takes in UniswapV3's sqrt_price_x96 (a q64_96 fixed point number) and outputs the price in human readable form.
/// See Uniswap's documentation: https://docs.uniswap.org/sdk/guides/fetching-prices
pub fn compute_price(tokens: (Token, Token), sqrt_price_x96: U256, pool_token_0: H160) -> BigFloat {
    let diff_decimals: BigFloat = ((tokens.0.decimals as i16) - (tokens.1.decimals as i16)).into();
    if pool_token_0 == tokens.0.address {
        utils::convert_q64_96(sqrt_price_x96)
            .pow(&BigFloat::from_i16(2))
            .div(&BigFloat::from_i16(10).pow(&-diff_decimals))
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
    use crate::{tokens, utils};
    use ethers::{abi::Address, providers::*};
    use std::sync::Arc;

    use super::Pool;

    #[tokio::test]
    async fn test_get_pool_from_uniswap() {
        let provider: Arc<Provider<Http>> = utils::get_provider().await;
        let tokens = tokens::get_tokens();

        let bp_1 = String::from("1");
        let bp_5 = String::from("5");
        let bp_30 = String::from("30");
        let bp_100 = String::from("100");

        let tokens = (
            tokens.get("ETH").unwrap().to_owned(),
            tokens.get("USDC").unwrap().to_owned(),
        );

        let pool_1 = Pool::new(
            tokens.0.clone(),
            tokens.1.clone(),
            bp_1.parse::<u32>().unwrap(),
            provider.clone(),
        )
        .await
        .unwrap();

        let pool_5 = Pool::new(
            tokens.0.clone(),
            tokens.1.clone(),
            bp_5.parse::<u32>().unwrap(),
            provider.clone(),
        )
        .await
        .unwrap();

        let pool_30 = Pool::new(
            tokens.0.clone(),
            tokens.1.clone(),
            bp_30.parse::<u32>().unwrap(),
            provider.clone(),
        )
        .await
        .unwrap();

        let pool_100 = Pool::new(
            tokens.0.clone(),
            tokens.1.clone(),
            bp_100.parse::<u32>().unwrap(),
            provider.clone(),
        )
        .await
        .unwrap();

        assert_eq!(
            pool_1.address,
            "0xe0554a476a092703abdb3ef35c80e0d76d32939f"
                .parse::<Address>()
                .unwrap()
        );

        assert_eq!(
            pool_5.address,
            "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640"
                .parse::<Address>()
                .unwrap()
        );

        assert_eq!(
            pool_30.address,
            "0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8"
                .parse::<Address>()
                .unwrap()
        );

        assert_eq!(
            pool_100.address,
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
        let bp = String::from("700");

        let tokens = (
            tokens.get("ETH").unwrap().to_owned(),
            tokens.get("USDC").unwrap().to_owned(),
        );

        // Creation should return an Err(()) and unwrap() should therefore panic.
        let _pool_700 = Pool::new(
            tokens.0.clone(),
            tokens.1.clone(),
            bp.parse::<u32>().unwrap(),
            provider.clone(),
        )
        .await
        .unwrap();
    }
}
