//! The `ArbiterDB` is a wrapper around a `CacheDB` that is used to provide
//! access to the `Environment`'s database to multiple `Coprocessors`.
//! It is also used to be able to write out the `Environment` database to a
//! file.

use std::{
    convert::Infallible,
    fmt::Debug,
    fs,
    io::{self, Read, Write},
    sync::{Arc, RwLock},
};

use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{AccountInfo, B256, U256},
    Database, DatabaseCommit,
};
use revm_primitives::{db::DatabaseRef, Bytecode};
use serde::{Deserialize, Serialize};
use serde_json;

/// A `ArbiterDB` is a wrapper around a `CacheDB` that is used to provide
/// access to the `Environment`'s database to multiple `Coprocessors`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArbiterDB(pub(crate) Arc<RwLock<CacheDB<EmptyDB>>>);

impl ArbiterDB {
    /// Create a new `ArbiterDB`.
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(CacheDB::new(EmptyDB::new()))))
    }

    /// Write the `ArbiterDB` to a file at the given path.
    pub fn write_to_file(&self, path: &str) -> io::Result<()> {
        // Serialize the ArbiterDB
        let serialized = serde_json::to_string(self)?;
        // Write to file
        let mut file = fs::File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    /// Read the `ArbiterDB` from a file at the given path.
    pub fn read_from_file(path: &str) -> io::Result<Self> {
        // Read the file content
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // Deserialize the content into ArbiterDB
        let cache_db = serde_json::from_str(&contents)?;
        Ok(Self(Arc::new(RwLock::new(cache_db))))
    }
}

impl Default for ArbiterDB {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: This is a BAD implementation of PartialEq, but it works for now as we
// do not ever really need to compare DBs directly at the moment.
// This is only used in the `Outcome` enum for `instruction.rs`.
impl PartialEq for ArbiterDB {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Database for ArbiterDB {
    type Error = Infallible; // TODO: Not sure we want this, but it works for now.

    fn basic(
        &mut self,
        address: revm::primitives::Address,
    ) -> Result<Option<AccountInfo>, Self::Error> {
        self.0.write().unwrap().basic(address)
    }

    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        self.0.write().unwrap().code_by_hash(code_hash)
    }

    fn storage(
        &mut self,
        address: revm::primitives::Address,
        index: U256,
    ) -> Result<U256, Self::Error> {
        self.0.write().unwrap().storage(address, index)
    }

    fn block_hash(&mut self, number: U256) -> Result<B256, Self::Error> {
        self.0.write().unwrap().block_hash(number)
    }
}

impl DatabaseRef for ArbiterDB {
    type Error = Infallible; // TODO: Not sure we want this, but it works for now.

    fn basic_ref(
        &self,
        address: revm::primitives::Address,
    ) -> Result<Option<AccountInfo>, Self::Error> {
        self.0.read().unwrap().basic_ref(address)
    }

    fn code_by_hash_ref(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        self.0.read().unwrap().code_by_hash_ref(code_hash)
    }

    fn storage_ref(
        &self,
        address: revm::primitives::Address,
        index: U256,
    ) -> Result<U256, Self::Error> {
        self.0.read().unwrap().storage_ref(address, index)
    }

    fn block_hash_ref(&self, number: U256) -> Result<B256, Self::Error> {
        self.0.read().unwrap().block_hash_ref(number)
    }
}

impl DatabaseCommit for ArbiterDB {
    fn commit(
        &mut self,
        changes: hashbrown::HashMap<revm::primitives::Address, revm::primitives::Account>,
    ) {
        self.0.write().unwrap().commit(changes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_to_file() {
        let db = ArbiterDB::new();
        db.write_to_file("test.json").unwrap();
        let db = ArbiterDB::read_from_file("test.json").unwrap();
        assert_eq!(db, ArbiterDB::new());
        fs::remove_file("test.json").unwrap();
    }
}
