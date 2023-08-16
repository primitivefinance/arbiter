#![allow(missing_docs)]

// mod interaction;
mod contracts;
mod interaction;
mod management;
mod signer;

use std::{str::FromStr, sync::Arc};

use anyhow::{Result};
use ethers::{
    prelude::{EthLogDecode, Middleware, StreamExt},
    types::{Address, Filter, ValueOrArray, U64},
};

use crate::{
    bindings::{arbiter_math::*, arbiter_token::*, liquid_exchange::*},
    environment::{tests::TEST_ENV_LABEL, *},
    manager::*,
    math::*,
    middleware::*,
};

pub const TEST_BLOCK_RATE: f64 = 2.0;
pub const TEST_ENV_SEED: u64 = 1;

pub const TEST_ARG_NAME: &str = "ArbiterToken";
pub const TEST_ARG_SYMBOL: &str = "ARBT";
pub const TEST_ARG_DECIMALS: u8 = 18;

pub const TEST_MINT_AMOUNT: u128 = 69;
pub const TEST_MINT_TO: &str = "0xf7e93cc543d97af6632c9b8864417379dba4bf15";

pub const TEST_APPROVAL_AMOUNT: u128 = 420;

pub const TEST_SIGNER_SEED_AND_LABEL: &str = "test_seed_and_label";

// TODO: Send a tx before and after pausing the environment.

async fn deploy_and_start() -> Result<(
    ArbiterToken<RevmMiddleware>,
    Environment,
    Arc<RevmMiddleware>,
)> {
    let mut environment = Environment::new(TEST_ENV_LABEL, TEST_BLOCK_RATE, TEST_ENV_SEED);
    let client = Arc::new(RevmMiddleware::new(
        &environment,
        Some(TEST_SIGNER_SEED_AND_LABEL.to_string()),
    ));
    environment.run();
    Ok((
        ArbiterToken::deploy(
            client.clone(),
            (
                TEST_ARG_NAME.to_string(),
                TEST_ARG_SYMBOL.to_string(),
                TEST_ARG_DECIMALS,
            ),
        )?
        .send()
        .await?,
        environment,
        client,
    ))
}
