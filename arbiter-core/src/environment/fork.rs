use std::{collections::HashMap, env, fs};

use ethers::types::Address;

use super::*;

#[derive(Clone, Debug)]
pub struct ForkedDB(pub CacheDB<EmptyDB>);

impl ForkedDB {
    pub fn from_disk(path: &str) -> Result<CacheDB<EmptyDB>, EnvironmentError> {
        // Read the file
        let mut cwd = env::current_dir().unwrap();
        cwd.push(path);
        println!("Reading db from: {:?}", cwd);
        let data = fs::read_to_string(cwd).unwrap();

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
pub struct DiskData(pub HashMap<Address, (AccountInfo, Storage)>);
