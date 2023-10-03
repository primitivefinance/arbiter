use std::env;
use std::str::FromStr;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
    sync::Arc,
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

use arbiter_core::environment::fork::*;

use super::*;

pub(crate) mod digest;
use digest::*;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForkConfig {
    pub output_directory: Option<String>,
    pub output_filename: Option<String>,
    pub provider: String,
    pub block_number: u64,
    pub contracts: HashMap<String, ContractData>,
}

impl ForkConfig {
    pub fn new(fork_config_path: &str) -> Result<Self, ConfigError> {
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

    pub fn to_db(&self) -> Result<ForkedDB, ConfigurationError> {
        // Spawn the `EthersDB` and the `CacheDB` we will write to.
        let mut ethers_db = self.spawn_ethers_db().unwrap();

        // Digest all of the contracts and their storage data listed in the fork config.
        let db = digest_config(self, &mut ethers_db).unwrap();

        Ok(ForkedDB(db))
    }

    pub(crate) fn to_disk(&self, overwrite: &bool) -> Result<(), ConfigurationError> {
        // Check if a file at the output path already exists.
        // These unwraps cannot fail.
        let path = Path::new(&self.output_directory.clone().unwrap())
            .join(self.output_filename.clone().unwrap());
        if path.exists() && path.is_file() {
            if !overwrite {
                // TODO: We should allow for an overwrite flag here.
                panic!(
                "File already exists at output path. Please use the `--overwrite` flag, delete it, or change the output path."
            );
            } else {
                fs::remove_file(path).unwrap();
            }
        }

        let forked_db = self.to_db().unwrap();
        let mut account_mapping = HashMap::new();
        for (address, db_account) in forked_db.0.accounts {
            let info = db_account.info;
            let mut storage = HashMap::new();
            for key in db_account.storage.keys() {
                let recast_key = key.to_string();
                let recast_value = db_account.storage.get(key).unwrap().to_string();
                storage.insert(recast_key, recast_value);
            }
            let address_as_bytes: [u8; 20] = address.as_bytes().try_into().unwrap();
            account_mapping.insert(Address::from(address_as_bytes), (info, storage));
        }
        let disk_data = DiskData(account_mapping);

        let json_data = serde_json::to_string(&disk_data).unwrap();

        // Create a directory in the CWD to store the CSV file.
        // The unwraps for the file storage cannot fail since they are set to defaults in the
        // `new` function.
        let output_directory = self.output_directory.clone().unwrap();
        fs::create_dir_all(&output_directory)?;
        let file_path = Path::new(&output_directory).join(self.output_filename.clone().unwrap());
        let mut file = fs::File::create(file_path)?;
        file.write_all(json_data.as_bytes()).unwrap();

        Ok(())
    }

    pub fn spawn_ethers_db(&self) -> Result<EthersDB<Provider<Http>>, ConfigurationError> {
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
