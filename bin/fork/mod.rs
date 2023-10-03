#![warn(missing_docs)]

use std::{collections::HashMap, env, fs, io::Write, path::Path, sync::Arc};

use arbiter_core::environment::fork::*;
use config::{Config, ConfigError};
use ethers::{
    providers::{Http, Provider},
    types::{Address, BlockId, BlockNumber, U256},
    utils::{hex, keccak256},
};
use revm::{
    db::{ethersdb::EthersDB, CacheDB, EmptyDB},
    Database,
};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use super::*;

pub(crate) mod digest;
#[cfg(test)]
mod tests;

/// A `ForkConfig` is a d
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ForkConfig {
    output_directory: Option<String>,
    output_filename: Option<String>,
    provider: String,
    block_number: u64,
    #[serde(rename = "contracts")]
    contracts_meta: HashMap<String, ContractMetadata>,
}

impl ForkConfig {
    pub(crate) fn new(fork_config_path: &str) -> Result<Self, ConfigError> {
        let mut cwd = env::current_dir().unwrap();
        cwd.push(fork_config_path);
        println!("Reading config from: {:?}", cwd);
        let config = Config::builder()
            .add_source(config::File::with_name(cwd.to_str().unwrap()))
            .build()?;
        let mut fork_config: ForkConfig = config.try_deserialize().unwrap();

        if fork_config.output_directory.is_none() {
            println!("No output path specified. Defaulting to current directory.");
            fork_config.output_directory = Some("./".to_string());
        }
        if fork_config.output_filename.is_none() {
            println!("No output filename specified. Defaulting to `output.json.`");
            fork_config.output_filename = Some("output.json".to_string());
        }

        Ok(fork_config)
    }

    /// Digests the config file and takes in an `EthersDB` so that the data can
    /// be fetched from the blockchain.
    /// Once all the `AccountInfo` for the contracts are fetched, we digest the
    /// contract artifacts to get the storage layout.
    pub(crate) fn digest_config(&self) -> Result<CacheDB<EmptyDB>, ConfigurationError> {
        // Spawn the `EthersDB` and the `CacheDB` we will write to.
        let ethers_db = &mut self.spawn_ethers_db().unwrap();
        let mut db = CacheDB::new(EmptyDB::default());
        for contract_data in self.contracts_meta.values() {
            let address = contract_data.address;
            let info = ethers_db.basic(address.into()).unwrap().unwrap();
            db.insert_account_info(address.into(), info);

            let artifacts =
                digest::digest_artifacts(contract_data.artifacts_path.as_str()).unwrap();
            let storage_layout = artifacts.storage_layout;

            digest::create_storage_layout(contract_data, storage_layout, &mut db, ethers_db)
                .unwrap();
        }
        Ok(db)
    }

    pub(crate) fn into_fork(self) -> Result<Fork, ConfigurationError> {
        // Digest all of the contracts and their storage data listed in the fork config.
        let db = self.digest_config().unwrap();

        Ok(Fork {
            db,
            contracts_meta: self.contracts_meta.clone(),
        })
    }

    pub(crate) fn write_to_disk(self, overwrite: &bool) -> Result<(), ConfigurationError> {
        // Check if a file at the output path already exists.
        // These unwraps cannot fail.
        let dir = self.output_directory.clone().unwrap();
        let file_path = Path::new(&self.output_directory.clone().unwrap())
            .join(self.output_filename.clone().unwrap());
        println!("path: {:?}", file_path);
        if file_path.exists() && file_path.is_file() {
            if !overwrite {
                // TODO: We should allow for an overwrite flag here.
                panic!(
                "File already exists at output path. Please use the `--overwrite` flag, delete it, or change the output path."
            );
            } else {
                fs::remove_file(&file_path).unwrap();
            }
        }

        let fork = self.into_fork().unwrap();
        let mut raw = HashMap::new();
        for (address, db_account) in fork.db.accounts {
            let info = db_account.info;
            let mut storage = HashMap::new();
            for key in db_account.storage.keys() {
                let recast_key = key.to_string();
                let recast_value = db_account.storage.get(key).unwrap().to_string();
                storage.insert(recast_key, recast_value);
            }
            let address_as_bytes: [u8; 20] = address.as_bytes().try_into().unwrap();
            raw.insert(Address::from(address_as_bytes), (info, storage));
        }
        let disk_data = DiskData {
            meta: fork.contracts_meta,
            raw,
        };

        let json_data = serde_json::to_string(&disk_data).unwrap();

        fs::create_dir_all(dir)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(json_data.as_bytes()).unwrap();

        Ok(())
    }

    fn spawn_ethers_db(&self) -> Result<EthersDB<Provider<Http>>, ConfigurationError> {
        let ethers_db = EthersDB::new(
            Arc::new(
                Provider::<Http>::try_from(self.provider.clone())
                    .expect("could not instantiate HTTP Provider"),
            ),
            Some(BlockId::Number(BlockNumber::Number(
                self.block_number.into(),
            ))),
        )
        .unwrap();
        Ok(ethers_db)
    }
}
