use std::{collections::HashMap, fs, sync::Arc};

use super::*;

use config::{Config, ConfigError};
use ethers::{
    providers::{Http, Provider},
    types::{Address, BlockId, BlockNumber, U256},
};
use revm::{
    db::{ethersdb::EthersDB, CacheDB, EmptyDB},
    Database,
};
use serde::{Deserialize, Serialize};

use arbiter_core::{environment, middleware::RevmMiddleware};
use ethers::utils::keccak256;

mod digest;
use digest::*;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct ForkedDb(CacheDB<EmptyDB>);

impl ForkedDb {
    pub fn new(fork_config: impl Into<String>) -> Result<Self, ConfigurationError> {
        // Read the fork config file.
        let fork_config = ForkConfig::new(&fork_config.into()).unwrap();

        // Check if a file at the output path already exists.
        // check_existing(&fork_config);

        // Spawn the `EthersDB` and the `CacheDB` we will write to.
        let mut ethers_db = spawn_ethers_db(&fork_config).unwrap();

        // Digest all of the contracts and their storage data listed in the fork config.
        let db = digest_config(&fork_config, &mut ethers_db).unwrap();

        Ok(ForkedDb(db))
    }
}

impl From<ForkedDb> for CacheDB<EmptyDB> {
    fn from(val: ForkedDb) -> Self {
        val.0
    }
}

pub fn check_existing(fork_config: &ForkConfig) -> Result<(), ConfigurationError> {
    todo!("check if the file exists and if it does, load it into the db")
}

pub fn spawn_ethers_db(
    fork_config: &ForkConfig,
) -> Result<EthersDB<Provider<Http>>, ConfigurationError> {
    let ethers_db = EthersDB::new(
        Arc::new(
            Provider::<Http>::try_from(fork_config.provider.clone())
                .expect("could not instantiate HTTP Provider"),
        ),
        Some(BlockId::Number(BlockNumber::Number(
            fork_config.block_number.into(),
        ))),
    )
    .unwrap();
    Ok(ethers_db)
}
