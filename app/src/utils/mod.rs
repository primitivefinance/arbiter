use crate::cli::get_cli;
use crate::tokens::{self, Token};
use ethers::prelude::*;
use ethers::providers::Provider;
use num_bigfloat::BigFloat; // TODO: Best to work with fixed point q64_96 for UniswapV3
use std::sync::Arc;

pub async fn get_provider() -> Arc<Provider<Http>> {
    let provider =
        Provider::try_from("https://eth-mainnet.g.alchemy.com/v2/I93POQk49QE9O-NuOz7nj7sbiluW76it")
            .unwrap();
    Arc::new(provider)
}

// Search through token list to get token objects from user input
pub fn get_tokens_from_cli() -> ((Token, Token), String) {
    let (token_0_string, token_1_string, bp, _api_key) = get_cli();
    let tokens = tokens::get_tokens();
    (
        (
            tokens.get(token_0_string.as_str()).unwrap().to_owned(),
            tokens.get(token_1_string.as_str()).unwrap().to_owned(),
        ),
        bp,
    )
}
pub fn convert_q64_96(q64_96: U256) -> BigFloat {
    // Take in a U256 structured as a q64_96 fixed point from UniswapV3 and converts this to a BigFloat.
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
