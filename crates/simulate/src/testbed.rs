use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{AccountInfo, B160, U256},
    EVM,
};

/// Uniswap V3 factory address.
const FACTORY: &str = "0x1F98431c8aD98523631AE4a59f267346ea31F984";

#[derive(Debug, Default)]
pub struct ExecutionManager {
    pub evm: EVM<CacheDB<EmptyDB>>,
}

impl ExecutionManager {
    /// Public constructor function to instantiate an `ExecutionManager`.
    pub fn new() -> Self {
        let mut evm = EVM::new();
        let mut db = CacheDB::new(EmptyDB {});

        evm.database(db);

        Self { evm }
    }

    /// Deploy a contract to the Execution instance.
    fn deploy_contract(&mut self, bytecode: Bytecode) {
        let factory_info = AccountInfo::new(U256::from(0), 0, bytecode);

        self.evm.db.insert_account_info(B160::from_str(FACTORY), factory_info);
    }

    /// Give an address a specified amount of ether.
    fn deal(&mut self, address: B160, amount: U256) {
        let account = self.load_account(address).unwrap();

        account.info.balance = amount;
    }
}