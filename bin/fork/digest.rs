use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForkConfig {
    pub provider: String,
    pub output_path: Option<String>, //TODO: Provide default storage locations based on name of config/block number
    pub filename: String,
    pub block_number: u64,
    pub contract_digests: Vec<ContractDigest>,
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
pub struct ContractDigest {
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

pub fn parse_artifacts(path: &str) -> Result<Artifacts, ConfigurationError> {
    // Read the file to a string
    let data = fs::read_to_string(path)?;

    // Parse the string into your WETH struct
    let json_data = serde_json::from_str(&data).unwrap();
    println!("json_data: {:?}", json_data);

    Ok(json_data)
}

pub fn digest() {
    todo!()
}

pub fn check_existing() -> Result<(), ConfigurationError> {
    todo!("check if the file exists and if it does, load it into the db")
}
