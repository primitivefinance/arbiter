use bindings::uniswap_v3_factory::UniswapV3Factory;
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
    let uniswapV3_Factory_address = "0x1F98431c8aD98523631AE4a59f267346ea31F984"
        .parse::<Address>()
        .unwrap();

    UniswapV3Factory::new(uniswapV3_Factory_address, provider.clone())
    // let balancer_vault = Vault::new(balancer_vault_address, provider.clone());
}
