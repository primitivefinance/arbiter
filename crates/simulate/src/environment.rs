use std::collections::HashMap;
use crate::agent::{Agent, SimulationManager};
use ethers::{abi::Tokenize, prelude::{BaseContract, Address}};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ruint::Uint, AccountInfo, ExecutionResult, Output, TransactTo, B160, U256, TxEnv},
    EVM,
};


pub struct SimulationEnvironment {
    pub evm: EVM<CacheDB<EmptyDB>>,
    pub agents: HashMap<String, Box<dyn Agent>>,
}

impl SimulationEnvironment {
    /// Public constructor function to instantiate an `ExecutionManager`.
    pub fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.database(db);

        Self {
            evm,
            agents: HashMap::new(), // This will only store agents that aren't the manager.
        }
    }

    /// Execute a transaction.
    // pub fn execute(
    //     &mut self,
    //     sender: B160,
    //     data: Vec<u8>,
    //     transact_to: TransactTo,
    //     value: U256,
    // ) -> ExecutionResult {
    //     self.evm.env.tx.caller = sender;
    //     self.evm.env.tx.transact_to = transact_to;
    //     self.evm.env.tx.data = data.into();
    //     self.evm.env.tx.value = value;
    pub fn execute(&mut self, tx: TxEnv) -> ExecutionResult {
        self.evm.env.tx = tx;
        match self.evm.transact_commit() {
            Ok(val) => val,
            // URGENT: change this to a custom error
            Err(_) => panic!("failed"),
        }
    }

        /// Create a user account.
        pub fn create_user(&mut self, address: B160) {
            self.evm
                .db()
                .unwrap()
                .insert_account_info(address, AccountInfo::default());
        }

    /// Give an address a specified amount of raw ether.
    pub fn deal(&mut self, address: B160, amount: U256) {
        let account = self.evm.db().unwrap().load_account(address).unwrap();

        account.info.balance += amount;
    }
}

/// Recast a B160 into an Address type (perhaps this should be in utils?)
pub fn recast_address(address: B160) -> Address {
    let temp: [u8;20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}