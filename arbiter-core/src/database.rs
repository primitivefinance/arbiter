use std::{
    convert::Infallible,
    fmt::Debug,
    sync::{Arc, RwLock},
};

use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{AccountInfo, B256, U256},
    Database, DatabaseCommit,
};
use revm_primitives::{db::DatabaseRef, Bytecode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArbiterDB(pub(crate) Arc<RwLock<CacheDB<EmptyDB>>>);

impl ArbiterDB {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(CacheDB::new(EmptyDB::new()))))
    }
}

// TODO: This is a BAD implementation of PartialEq, but it works for now as we
// do not ever really need to compare DBs directly at the moment.
// This is only used in the `Outcome` enum for `instruction.rs`.
impl PartialEq for ArbiterDB {
    fn eq(&self, other: &Self) -> bool {
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
