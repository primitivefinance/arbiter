use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
    sync::Arc,
};

use arbiter_core::{
    environment,
    middleware::{cast::recast_address, RevmMiddleware},
};
use config::{Config, ConfigError};
use ethers::{
    providers::{Http, Provider},
    types::{Address, BlockId, BlockNumber, U256},
    utils::{hex, keccak256},
};
use revm::{
    db::{ethersdb::EthersDB, CacheDB, EmptyDB},
    primitives::AccountInfo,
    Database,
};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use super::*;

mod digest;
use digest::*;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct ForkedDB(CacheDB<EmptyDB>);

impl ForkedDB {
    pub fn new(fork_config: &ForkConfig) -> Result<Self, ConfigurationError> {
        // Spawn the `EthersDB` and the `CacheDB` we will write to.
        let mut ethers_db = spawn_ethers_db(fork_config).unwrap();

        // Digest all of the contracts and their storage data listed in the fork config.
        let db = digest_config(fork_config, &mut ethers_db).unwrap();

        Ok(ForkedDB(db))
    }

    pub fn from_disk(path: &str) -> Result<CacheDB<EmptyDB>, ConfigurationError> {
        // Read the file
        let data = fs::read_to_string(path)?;

        // Deserialize the JSON data to your OutputData type
        let disk_data: DiskData = serde_json::from_str(&data).unwrap();

        // Create a CacheDB instance
        let mut db = CacheDB::new(EmptyDB::default());

        // Populate the CacheDB from the OutputData
        for (address, (info, storage_map)) in disk_data.0 {
            // Convert the string address back to its original type
            let address = address.into(); // You'd need to define this

            // Insert account info into the DB
            db.insert_account_info(address, info);

            // Insert storage data into the DB
            for (key_str, value_str) in storage_map {
                let key = revm::primitives::U256::from_str_radix(&key_str, 10).unwrap();
                let value = revm::primitives::U256::from_str_radix(&value_str, 10).unwrap();

                db.insert_account_storage(address, key, value);
            }
        }

        Ok(db)
    }
}

impl From<ForkedDB> for CacheDB<EmptyDB> {
    fn from(val: ForkedDB) -> Self {
        val.0
    }
}

type Storage = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
struct DiskData(HashMap<Address, (AccountInfo, Storage)>);

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
