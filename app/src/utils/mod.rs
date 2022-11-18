use bindings::uniswap_v3_factory::UniswapV3Factory;
use ethers::abi::Address;
use ethers::prelude::*;
use ethers::providers::Provider;
use num_bigfloat::BigFloat;
use std::sync::Arc;

// pub const two: BigFloat = 2.0.into();
// pub const ten: BigFloat = 10.0.into();
pub fn convert(q64_96: U256) -> BigFloat {
    let least_sig = q64_96.0[0];
    let second_sig = q64_96.0[1];
    let third_sig = q64_96.0[2];
    let most_sig = q64_96.0[3];

    let bf2 = BigFloat::from(2);
    let bf64 = BigFloat::from(64);
    let bf128 = BigFloat::from(128);
    let bf192 = BigFloat::from(192);
    let bf96 = BigFloat::from(96);

    ((BigFloat::from(most_sig) * bf2.pow(&bf192))
        + (BigFloat::from(third_sig) * bf2.pow(&bf128))
        + (BigFloat::from(second_sig) * bf2.pow(&bf64))
        + BigFloat::from(least_sig))
        / bf2.pow(&bf96)
}
pub async fn get_provider() -> Arc<Provider<Http>> {
    let provider =
        Provider::try_from("https://eth-mainnet.g.alchemy.com/v2/I93POQk49QE9O-NuOz7nj7sbiluW76it")
            .unwrap();
    Arc::new(provider)
}
pub async fn get_uniswapv3_factory(
    provider: Arc<Provider<Http>>,
) -> UniswapV3Factory<Provider<Http>> {
    let uniswap_v3_factory_address = "0x1F98431c8aD98523631AE4a59f267346ea31F984"
        .parse::<Address>()
        .unwrap();
    UniswapV3Factory::new(uniswap_v3_factory_address, provider.clone())
}
pub async fn get_pool_from_uniswap(
    token_0: Address,
    token_1: Address,
    factory: UniswapV3Factory<Provider<Http>>,
) -> Vec<Address> {
    // BP 10000, 3000, 500, 100
    let pool_500 = factory
        .get_pool(token_0, token_1, 500)
        .call()
        .await
        .unwrap();
    let pool_100 = factory
        .get_pool(token_0, token_1, 100)
        .call()
        .await
        .unwrap();
    let pool_3000 = factory
        .get_pool(token_0, token_1, 3000)
        .call()
        .await
        .unwrap();
    let pool_10000 = factory
        .get_pool(token_0, token_1, 10000)
        .call()
        .await
        .unwrap();
    let mut pools = vec![pool_100, pool_500, pool_3000, pool_10000];
    let length = pools.len();
    for index in 0..length {
        if pools[index]
            != "0x0000000000000000000000000000000000000000"
                .parse::<Address>()
                .unwrap()
        {
            pools.remove(index);
        }
    }
    pools
}
