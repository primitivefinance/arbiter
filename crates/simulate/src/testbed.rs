use std::collections::HashMap;

use ethers::types::{BlockId, H160 as eH160, H256, U256 as eU256};
use revm::{
    db::{CacheDB, DatabaseRef, EmptyDB},
    AccountInfo, Database, Env, InMemoryDB, TransactOut, TransactTo, EVM, KECCAK_EMPTY, ExecutionResult,
};
use tokio::runtime::{Handle, Runtime};
use bytes::Bytes;

pub struct Testbed {
    pub db: InMemoryDB,
    pub runtime: Option<Runtime>,
    pub evm: EVM<CacheDB<EmptyDB>>,
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
            evm: EVM::new(),
        }
    }
    pub fn block_on<F: core::future::Future>(&self, f: F) -> F::Output {
        match &self.runtime {
            Some(runtime) => runtime.block_on(f),
            None => futures::executor::block_on(f),
        }
    }

    // pub fn perform_transaction(&self, addr_caller: eH160, addr_transact_to: eH160, data: Bytes) {
    //     self.evm.database(self.db);
    //     self.evm.env.tx.caller = addr_caller;
    //     self.evm.env.tx.transact_to = TransactTo::Call(addr_transact_to);
    //     self.evm.env.tx.data = Bytes::from(hex::decode(hex::encode(&data)));
    //     self.evm.env.tx.value = eU256::from(0);
    //     let ref_tx = self.evm.transact_ref();
    // }

}

