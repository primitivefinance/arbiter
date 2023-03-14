use bytes::Bytes;
use ethers::{
    abi::{Abi, Address, Tokenizable, Tokenize},
    prelude::BaseContract,
};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ruint::Uint, ExecutionResult, TransactTo, B160, U256},
    EVM,
};

#[derive(Debug)]
pub struct NotDeployed;
#[derive(Debug)]
pub struct IsDeployed;

#[derive(Debug)]
pub struct SimulationContract<Deployed> {
    pub base_contract: BaseContract,
    pub bytecode: Vec<u8>,
    pub address: Option<B160>,
    pub deployed: std::marker::PhantomData<Deployed>,
}

impl SimulationContract<NotDeployed> {
    pub fn new(base_contract: BaseContract, bytecode: Vec<u8>) -> Self {
        Self {
            base_contract,
            bytecode,
            address: None,
            deployed: std::marker::PhantomData,
        }
    }

    fn to_deployed(self, address: B160) -> SimulationContract<IsDeployed> {
        SimulationContract {
            base_contract: self.base_contract,
            bytecode: self.bytecode,
            address: Some(address),
            deployed: std::marker::PhantomData,
        }
    }
}

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
        sender: B160,
        data: Vec<u8>,
        transact_to: TransactTo,
        value: U256,
    ) -> ExecutionResult {
        self.evm.env.tx.caller = sender;
        self.evm.env.tx.transact_to = transact_to;
        self.evm.env.tx.data = data.into();
        self.evm.env.tx.value = value;

        match self.evm.transact_commit() {
            Ok(val) => val,
            // URGENT: change this to a custom error
            Err(_) => panic!("failed"),
        }
    }

    /// Deploy a contract.
    pub fn deploy<T: Tokenizable>(
        &mut self,
        sender: B160,
        contract: SimulationContract<NotDeployed>,
        args: T,
    ) -> SimulationContract<IsDeployed> {
        let args = args.into_tokens();
        // Append to generate the deploy bytecode;
        let bytecode = contract
            .base_contract
            .abi()
            .constructor()
            .unwrap()
            .encode_input(contract.bytecode.clone(), &args)
            .unwrap(); // TODO: Need to catch this if error

        self.execute(sender, bytecode, TransactTo::create(), Uint::from(0));

        let contract_address = self
            .evm
            .db()
            .unwrap()
            .clone()
            .accounts
            .into_iter()
            .nth(2)
            .unwrap()
            .0;

        contract.to_deployed(contract_address)
    }

    /// Give an address a specified amount of raw ether.
    pub fn deal(&mut self, address: B160, amount: U256) {
        let account = self.evm.db().unwrap().load_account(address).unwrap();

        account.info.balance = amount;
    }
}
