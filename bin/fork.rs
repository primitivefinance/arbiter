use std::{env, fs, io::Empty, sync::Arc};

use super::*;

use config::{Config, ConfigError};
use ethers::{
    providers::{Http, Provider},
    types::{Address, Block, BlockId, BlockNumber},
};
use revm::{
    db::{ethersdb::EthersDB, CacheDB, EmptyDB},
    Database, InMemoryDB,
};
use serde::{Deserialize, Serialize};

use arbiter_core::environment::Environment;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ForkConfig {
    output_path: String,
    filename: String,
    block_number: u64,
    addresses: Vec<Address>,
}

impl ForkConfig {
    pub fn new(fork_config: &String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(fork_config))
            .build()?;
        s.try_deserialize()
    }
}
#[test]
// pub fn fork(fork_config: &String) -> Result<(), ConfigurationError> {
pub fn fork_write_out() -> Result<(), ConfigurationError> {
    let fork_config = &"fork_config.toml".to_owned();
    let fork_config = ForkConfig::new(fork_config).unwrap();
    let client = Arc::new(
        Provider::<Http>::try_from("https://eth.llamarpc.com")
            .expect("could not instantiate HTTP Provider"),
    );
    let mut test_db = CacheDB::new(EmptyDB::default());
    let block_id = BlockId::Number(BlockNumber::Number(fork_config.block_number.into()));
    let mut db = EthersDB::new(client, Some(block_id)).unwrap();
    for address in fork_config.addresses {
        let thing = db.basic(address.into()).unwrap().unwrap();
        test_db.insert_contract(&mut thing.clone());
        println!("The thing itself has:: {}: {:?}", address, thing);
        println!("The DB itself is: {:?}", test_db);
    }
    println!("Outputting to: {:?}", fork_config.output_path);
    let current_dir = env::current_dir()?;
    let output_dir = current_dir.join(fork_config.output_path);
    fs::create_dir_all(&output_dir)?;
    let file_path = output_dir.join(fork_config.filename);
    serde_json::to_writer_pretty(
        std::fs::File::create(file_path).unwrap(),
        &test_db.contracts,
    );
    Ok(())
}

#[test]
// pub fn fork(fork_config: &String) -> Result<(), ConfigurationError> {
pub fn fork_use_in_arbiter_core() -> Result<(), ConfigurationError> {
    let fork_config = &"fork_config.toml".to_owned();
    let fork_config = ForkConfig::new(fork_config).unwrap();
    let client = Arc::new(
        Provider::<Http>::try_from("https://eth.llamarpc.com")
            .expect("could not instantiate HTTP Provider"),
    );
    let mut test_db = CacheDB::new(EmptyDB::default());
    let block_id = BlockId::Number(BlockNumber::Number(fork_config.block_number.into()));
    let mut db = EthersDB::new(client, Some(block_id)).unwrap();
    for address in fork_config.addresses {
        let thing = db.basic(address.into()).unwrap().unwrap();
        test_db.insert_contract(&mut thing.clone());
    }

    Ok(())
}
