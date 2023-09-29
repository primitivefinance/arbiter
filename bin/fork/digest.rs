use std::ops::Add;

use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForkConfig {
    pub output_path: Option<String>, //TODO: Provide default storage locations based on name of config/block number
    pub output_filename: String,
    pub provider: String,
    pub block_number: u64,
    pub contracts: HashMap<String, ContractData>,
}

impl ForkConfig {
    pub fn new(fork_config: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(fork_config))
            .build()?;
        s.try_deserialize()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContractData {
    pub address: Address,
    pub artifacts_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artifacts {
    #[serde(rename = "storageLayout")]
    pub storage_layout: StorageLayout,
    // TODO: Add more here if we need them.
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StorageLayout {
    pub storage: Vec<StorageItem>,
    pub types: HashMap<String, StorageType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StorageItem {
    #[serde(rename = "astId")]
    pub ast_id: usize,
    pub contract: String,
    pub label: String,
    pub offset: usize,
    pub slot: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StorageType {
    Simple {
        encoding: String,
        label: String,
        #[serde(rename = "numberOfBytes")]
        number_of_bytes: String,
    },
    Mapping {
        encoding: String,
        key: String,
        value: String,
        label: Option<String>,
        #[serde(rename = "numberOfBytes")]
        number_of_bytes: Option<String>,
    },
}

/// Digests the config file and takes in an `EthersDB` so that the data can be fetched from the
/// blockchain.
/// Once all the `AccountInfo` for the contracts are fetched, we digest the contract artifacts to
/// get the storage layout.
pub fn digest_config(
    fork_config: &ForkConfig,
    ethers_db: &mut EthersDB<Provider<Http>>,
) -> Result<CacheDB<EmptyDB>, ConfigurationError> {
    let mut db = CacheDB::new(EmptyDB::default());
    for contract_digest in fork_config.contracts.values() {
        let address = contract_digest.address;
        let info = ethers_db.basic(address.into()).unwrap().unwrap();
        db.insert_account_info(address.into(), info);

        let artifacts = digest_artifacts(contract_digest.artifacts_path.as_str()).unwrap();
        let storage_layout = artifacts.storage_layout;

        create_storage_layout(address, storage_layout, &mut db, ethers_db).unwrap();
    }
    Ok(db)
}

pub fn digest_artifacts(path: &str) -> Result<Artifacts, ConfigurationError> {
    // Read the file to a string
    let data = fs::read_to_string(path)?;

    // Parse the string into your WETH struct
    let json_data = serde_json::from_str(&data).unwrap();
    println!("json_data: {:?}", json_data);

    Ok(json_data)
}

pub fn create_storage_layout(
    address: Address,
    storage_layout: StorageLayout,
    db: &mut CacheDB<EmptyDB>,
    ethers_db: &mut EthersDB<Provider<Http>>,
) -> Result<(), ConfigurationError> {
    for storage_item in storage_layout.storage {
        let slot = storage_item.slot;
        let slot_bytes = U256::from_str_radix(slot.as_str(), 16).unwrap();
        let storage = ethers_db
            .storage(address.into(), slot_bytes.into())
            .unwrap();
        db.insert_account_storage(address.into(), slot_bytes.into(), storage)
            .unwrap();
    }
    Ok(())
}
