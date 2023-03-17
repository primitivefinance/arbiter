use std::collections::HashMap;

use ethers::prelude::Address;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{AccountInfo, ExecutionResult, TxEnv, B160, U256},
    EVM,
};

use crate::agent::Agent;

pub struct SimulationEnvironment {
    pub evm: EVM<CacheDB<EmptyDB>>,
    pub agents: HashMap<String, Box<dyn Agent>>,
}

impl Default for SimulationEnvironment {
    /// Public default constructor function to instantiate an `ExecutionManager`.
    fn default() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.database(db);

        Self {
            evm,
            agents: HashMap::new(), // This will only store agents that aren't the manager.
        }
    }
}

impl SimulationEnvironment {
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
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}
