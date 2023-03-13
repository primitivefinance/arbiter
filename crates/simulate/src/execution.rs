use bytes::Bytes;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, TransactTo, B160, U256},
    EVM,
};

#[derive(Default)]
pub struct ExecutionManager {
    pub evm: EVM<CacheDB<EmptyDB>>,
}

impl ExecutionManager {
    /// Public constructor function to instantiate an `ExecutionManager`.
    pub fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});

        evm.database(db);

        Self { evm }
    }

    /// Execute a transaction.
    pub fn execute(
        &mut self,
        caller: B160,
        data: Bytes,
        transact_to: TransactTo,
        value: U256,
    ) -> ExecutionResult {
        self.evm.env.tx.caller = caller;
        self.evm.env.tx.transact_to = transact_to;
        self.evm.env.tx.data = data;
        self.evm.env.tx.value = value;

        match self.evm.transact_commit() {
            Ok(val) => val,
            // URGENT: change this to a custom error
            Err(_) => panic!("failed"),
        }
    }

    /// Give an address a specified amount of raw ether.
    pub fn deal(&mut self, address: B160, amount: U256) {
        let account = self.evm.db().unwrap().load_account(address).unwrap();

        account.info.balance = amount;
    }
}
