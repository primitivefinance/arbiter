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

        // Spawn the `EthersDB` and the `CacheDB` we will write to.
        let mut ethers_db = spawn_ethers_db(&fork_config).unwrap();
        let mut db = CacheDB::new(EmptyDB::default());

        // For each contract in the fork config, get the account info and storage.
        for contract_digest in fork_config.contract_digests.clone() {
            let address = contract_digest.address;
            let info = ethers_db.basic(address.into()).unwrap().unwrap();
            db.insert_account_info(address.into(), info);

            let artifacts = parse_artifacts(contract_digest.artifacts_path.as_str()).unwrap();
            let storage_layout = artifacts.storage_layout;

            for storage_item in storage_layout.storage {
                let slot = storage_item.slot;
                let slot_bytes = U256::from_str_radix(slot.as_str(), 16).unwrap();
                let storage = ethers_db
                    .storage(address.into(), slot_bytes.into())
                    .unwrap();
                db.insert_account_storage(address.into(), slot_bytes.into(), storage)
                    .unwrap();
            }
        }
        Ok(ForkedDb(db))
    }
}

impl From<ForkedDb> for CacheDB<EmptyDB> {
    fn from(val: ForkedDb) -> Self {
        val.0
    }
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
