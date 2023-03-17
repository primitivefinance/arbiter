//! ## Clairvoyance
//!
//! Clairvoyance is the monitoring, modelling and simulation suite of Arbiter.

use std::sync::Arc;

use bindings_old::{i_uniswap_v3_pool::IUniswapV3Pool, uniswap_v3_factory::UniswapV3Factory};
use ethers::{abi::Address, prelude::*, providers::Provider, types::H160};
use num_bigfloat::BigFloat;
#[allow(warnings)]
use utils::{
    chain_tools::convert_q64_96,
    tokens::{get_tokens, Token},
};

#[allow(warnings)]
use crate::error::UniswapError;

/// Uniswap V3 factory address.
const FACTORY: &str = "0x1F98431c8aD98523631AE4a59f267346ea31F984";

/// Representation of a pool.
#[derive(Debug, Clone)]
#[deprecated(since = "0.0.1", note = "will be useful for agents in the future")]
#[allow(warnings)]
pub struct Pool {
    /// Token 0.
    token_0: Token,
    /// Token 1.
    token_1: Token,
    /// Basis point of pool.
    bp: u32,
    /// Address of the pool.
    address: H160,
    /// Factory that created the pool. This could be generic in future.
    factory: UniswapV3Factory<Provider<Http>>,
    /// Pool contract object.
    inner: IUniswapV3Pool<Provider<Http>>,
    /// Current Tick.
    tick: i32,
    /// Current liquidity.
    liquidity: u128,
    /// sqrt_price_x96
    sqrt_price_x96: ethers::types::U256,
}
#[allow(warnings)]
impl Pool {
    /// Public constructor function that instantiates a `Pool`.
    pub async fn new(
        token_0: Token,
        token_1: Token,
        bp: u32,
        provider: Arc<Provider<Http>>,
    ) -> Result<Pool, UniswapError> {
        match bp {
            1 | 5 | 30 | 100 => (),
            _ => return Err(UniswapError::FeeTierError),
        }

        // Factory Address.
        let uniswap_v3_factory_address = FACTORY.parse::<Address>().unwrap();

        // Factory contract object.
        let factory = UniswapV3Factory::new(uniswap_v3_factory_address, provider.clone());

        let pool_address = match factory
            .get_pool(token_0.address, token_1.address, bp * 100)
            .call()
            .await
        {
            Ok(val) => val,
            Err(err) => return Err(UniswapError::ContractInteractionError(err)),
        };

        if pool_address == Address::zero() {
            return Err(UniswapError::PoolError);
        }

        Ok(Pool {
            token_0,
            token_1,
            bp,
            address: pool_address,
            factory,
            inner: IUniswapV3Pool::new(pool_address, provider.clone()),
            tick: 0,
            liquidity: 0,
            sqrt_price_x96: ethers::types::U256::zero(),
        })
    }

    /// Get the pool address.
    /// We need getters and setters as the internal state cannot be corrupted.
    pub fn get_address(&self) -> H160 {
        self.address
    }

    /// Get the tick of the pool.
    pub fn get_tick(&self) -> i32 {
        self.tick
    }

    /// Get the liquidity of the pool.
    pub fn get_liquidity(&self) -> u128 {
        self.liquidity
    }

    /// Get the two corresponding pool tokens.
    pub fn get_tokens(&self) -> (Token, Token) {
        (self.token_0.clone(), self.token_1.clone())
    }

    /// Get the pool basis points.
    /// The possible values are 5, 30 and 100.
    pub fn get_bp(&self) -> u32 {
        self.bp
    }

    /// Get the pool factory.
    pub fn get_factory(&self) -> UniswapV3Factory<Provider<Http>> {
        self.factory.clone()
    }

    /// Get the pool contract.
    pub fn get_contract(&self) -> IUniswapV3Pool<Provider<Http>> {
        self.inner.clone()
    }

    /// Get the pool sqrt_price.
    /// More information regarding how to derive pricing can be found in the whitepaper.
    pub fn get_sqrt_price_x96(&self) -> ethers::types::U256 {
        self.sqrt_price_x96
    }
    fn set_tick(&mut self, tick: i32) {
        self.tick = tick;
    }
    fn set_liquidity(&mut self, liquidity: u128) {
        self.liquidity = liquidity;
    }
    fn set_sqrt_price_x96(&mut self, sqrt_price_x96: ethers::types::U256) {
        self.sqrt_price_x96 = sqrt_price_x96;
    }

    /// Updates the pool tick and liquidity manually with a contract call.
    pub async fn _update_pool(&mut self) -> Result<(), UniswapError> {
        let slot_0 = match self.inner.slot_0().call().await {
            Err(err) => return Err(UniswapError::ContractInteractionError(err)),
            Ok(val) => val,
        };

        let liquidity = match self.inner.liquidity().call().await {
            Err(err) => return Err(UniswapError::ContractInteractionError(err)),
            Ok(val) => val,
        };

        self.set_liquidity(liquidity);
        self.set_tick(slot_0.1);
        self.set_sqrt_price_x96(slot_0.0);

        Ok(())
    }

    /// Monitor a pool for swap events and print to standard output.
    /// TODO: Make it print a `Swap` struct that implements fmt in a special way.
    pub async fn monitor_pool(&mut self) {
        let pool_contract = self.get_contract();
        let pool_tokens = self.get_tokens();
        let pool_bp = self.get_bp();

        println!(
            "...Got Pool (token0, token1, bps, address): {}, {}, {}, {:#?}\n",
            pool_tokens.0.name,
            pool_tokens.1.name,
            pool_bp,
            pool_contract.address()
        );

        println!("Listening for events...");

        // let pool_token_0 = match pool_contract.token_0().call().await {
        //     Err(err) => return Err(UniswapError::ContractInteractionError(err)),
        //     Ok(val) => val,
        // };

        let swap_events = pool_contract.swap_filter();
        let pool_token_0 = pool_contract.token_0().call().await.unwrap();
        let mut swap_stream = swap_events.stream().await.unwrap();

        while let Some(Ok(event)) = swap_stream.next().await {
            let (tick, liq, sqrtprice) = (event.tick, event.liquidity, event.sqrt_price_x96);
            self.set_tick(tick);
            self.set_liquidity(liq);
            self.set_sqrt_price_x96(sqrtprice);
            println!("------------NEW SWAP------------");
            println!("Pool:      {:#?}", pool_contract.address());
            println!("Sender:    {:#?}", event.sender);
            println!("Recipient: {:#?}", event.recipient);
            println!("Amount_0:  {:#?}", event.amount_0); // I256
            println!("Amount_1:  {:#?}", event.amount_1); // I256
            println!("Liquidity: {:#?}", event.liquidity); // u128
            println!("Tick:      {:#?}", event.tick); // i32
            println!(
                "Price:     {:#?}",
                compute_price(pool_tokens.clone(), event.sqrt_price_x96, pool_token_0,).to_string()
            );

            // Check tick, price, and liquidity where updated
            assert_eq!(event.tick, self.get_tick());
            assert_eq!(event.liquidity, self.get_liquidity());
            assert_eq!(event.sqrt_price_x96, self.get_sqrt_price_x96());
        }

        // Ok(())
    }

    /// Calculate the amount you would have to swap in order to have a swap that causes
    /// a given price impact.
    pub fn price_impact() {
        todo!()
    }
}

/// Wrapper function to easily create a pool.
#[allow(warnings)]
pub async fn get_pool(
    token0: &String,
    token1: &String,
    bp: &str,
    provider: Arc<Provider<Http>>,
) -> Result<Pool, UniswapError> {
    let tokens = get_tokens();

    let token0 = match tokens.get(token0) {
        Some(token) => token,
        None => return Err(UniswapError::TokenError),
    };

    let token1 = match tokens.get(token1) {
        Some(token) => token,
        None => return Err(UniswapError::TokenError),
    };

    let bp = bp.parse::<u32>().unwrap();

    Ok(Pool::new(token0.clone(), token1.clone(), bp, provider)
        .await
        .unwrap())
}

/// Get a sample test pool.
#[allow(warnings)]
pub async fn _get_test_pool(bp: String, provider: Arc<Provider<Http>>) -> Pool {
    let tokens = get_tokens();
    Pool::new(
        tokens.get("ETH").unwrap().to_owned(),
        tokens.get("DAI").unwrap().to_owned(),
        bp.parse::<u32>().unwrap(),
        provider,
    )
    .await
    .unwrap()
}

/// Takes in UniswapV3's sqrt_price_x96 (a q64_96 fixed point number) and outputs the price in human readable form.
/// See Uniswap's documentation: <https://docs.uniswap.org/sdk/guides/fetching-prices>
#[allow(warnings)]
pub fn compute_price(tokens: (Token, Token), sqrt_price_x96: U256, pool_token_0: H160) -> BigFloat {
    let diff_decimals: BigFloat = ((tokens.0.decimals as i16) - (tokens.1.decimals as i16)).into();
    if pool_token_0 == tokens.0.address {
        convert_q64_96(sqrt_price_x96)
            .pow(&BigFloat::from_i16(2))
            .div(&BigFloat::from_i16(10).pow(&-diff_decimals))
    } else {
        BigFloat::from_i16(1).div(
            &convert_q64_96(sqrt_price_x96)
                .pow(&BigFloat::from_i16(2))
                .div(&BigFloat::from_i16(10).pow(&diff_decimals)),
        )
    }
}

#[cfg(test)]
#[allow(warnings)]
mod tests {
    use std::sync::Arc;

    use ethers::{abi::Address, providers::*};
    use utils::{chain_tools, tokens};

    use super::Pool;

    macro_rules! create_pool {
        (
            $provider:expr,
            $tokens:expr,
            $bp:expr,
            $address:expr
        ) => {
            let pool = Pool::new($tokens.0.clone(), $tokens.1.clone(), $bp, $provider.clone())
                .await
                .unwrap();

            assert_eq!(pool.address, $address.parse::<Address>().unwrap());
        };
    }

    #[tokio::test]
    async fn test_get_pool_from_uniswap() {
        let provider: Arc<Provider<Http>> = chain_tools::get_provider().await;

        let tokens = (
            tokens::get_tokens().get("ETH").unwrap().to_owned(),
            tokens::get_tokens().get("USDC").unwrap().to_owned(),
        );

        create_pool!(
            provider,
            tokens,
            1,
            "0xe0554a476a092703abdb3ef35c80e0d76d32939f"
        );
        create_pool!(
            provider,
            tokens,
            5,
            "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640"
        );
        create_pool!(
            provider,
            tokens,
            30,
            "0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8"
        );
        create_pool!(
            provider,
            tokens,
            100,
            "0x7bea39867e4169dbe237d55c8242a8f2fcdcc387"
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn test_get_pool_from_uniswap_700() {
        let provider: Arc<Provider<Http>> = chain_tools::get_provider().await;

        let tokens = (
            tokens::get_tokens().get("ETH").unwrap().to_owned(),
            tokens::get_tokens().get("USDC").unwrap().to_owned(),
        );

        // This address is arbitrary as pool creation should anyways fail.
        create_pool!(
            provider,
            tokens,
            700,
            "0x7bea39867e4169dbe237d55c8242a8f2fcdcc387"
        );
    }
}
