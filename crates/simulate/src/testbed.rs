use ethers_core::types::H160 as eH160;
use revm::{
    db::{CacheDB, EmptyDB},
    EVM,
};
use tokio::runtime::{Handle, Runtime};

pub struct Testbed {
    // pub db: InMemoryDB,
    pub runtime: Option<Runtime>,
    pub evm: EVM<CacheDB<EmptyDB>>,
}

impl Default for Testbed {
    fn default() -> Self {
        Self::new()
    }
}

impl Testbed {
    pub fn new() -> Self {
        let db = CacheDB::new(EmptyDB {});
        let mut evm = EVM::new();
        evm.database(db);
        Testbed {
            runtime: Handle::try_current()
                .is_err()
                .then(|| Runtime::new().unwrap()),
            evm,
        }
    }
    pub fn block_on<F: core::future::Future>(&self, f: F) -> F::Output {
        match &self.runtime {
            Some(runtime) => runtime.block_on(f),
            None => futures::executor::block_on(f),
        }
    }

    pub fn create_user(&mut self, addr: eH160) {
        let info = revm::AccountInfo::default();
        self.evm.db().unwrap().insert_account_info(addr, info);
    }
}
