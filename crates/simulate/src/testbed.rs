use revm::{
    primitives::{AccountInfo,B160},
    db::{CacheDB, EmptyDB},
    EVM,
};
use tokio::runtime::{Handle, Runtime};

pub struct Testbed {
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

    pub fn create_user(&mut self, addr: B160) {
        let info = AccountInfo::default();
        self.evm.db().unwrap().insert_account_info(addr, info);
    }

    // TODO: Implement a create_contract() fxn
}
