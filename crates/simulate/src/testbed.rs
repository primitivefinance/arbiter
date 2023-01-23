use revm::{
    db::{CacheDB, DatabaseRef, EmptyDB},
    AccountInfo, Database, Env, InMemoryDB, TransactOut, TransactTo, EVM, KECCAK_EMPTY,
};
use tokio::runtime::{Handle, Runtime};

pub struct Testbed {
    pub db: InMemoryDB,
    pub runtime: Option<Runtime>,
}

impl Testbed {
    // pub fn get_db(&self) -> InMemoryDB {
    //     self.db
    // }

    // pub fn get_runtime(&self) -> Option<Runtime> {
    //     self.runtime
    // }

    pub fn new() -> Self {
        Testbed {
            db: CacheDB::new(EmptyDB {}),
            runtime: Handle::try_current()
                .is_err()
                .then(|| Runtime::new().unwrap()),
        }
    }
    pub fn block_on<F: core::future::Future>(&self, f: F) -> F::Output {
        match &self.runtime {
            Some(runtime) => runtime.block_on(f),
            None => futures::executor::block_on(f),
        }
    }
}