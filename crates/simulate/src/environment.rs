use crate::agent::{Agent, Admin};
use ethers::{abi::Tokenize, prelude::{BaseContract, Address}};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ruint::Uint, AccountInfo, ExecutionResult, Output, TransactTo, B160, U256},
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
    pub address: Option<B160>, //TODO: Options may not be the best thing here. Also, B160 might not and Address=H160 might be. 
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

    fn to_deployed(&self, address: B160) -> SimulationContract<IsDeployed> {
        SimulationContract {
            base_contract: self.base_contract.clone(),
            bytecode: self.bytecode.clone(),
            address: Some(address),
            deployed: std::marker::PhantomData,
        }
    }
}

// #[derive(Default)] // Not sure this was ever necessary
pub struct SimulationEnvironment {
    pub evm: EVM<CacheDB<EmptyDB>>,
    pub admin: Admin,
    pub agents: Vec<Box<dyn Agent>>,
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
            admin: Admin::new(),
            agents: vec![], // TODO: This should be a hashmap of agents? 
        }
    }

    /// Execute a transaction.
    /// TODO: An agent should be making calls, but the calls should probably just call execute?
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

    /// Deploy a contract. We will assume the sender is always the admin.
    /// TODO: This should call `recast_address` when a B160 is passed as an arg. Not sure how to handle this yet.
    /// Given the above comments there should be a nice way to do this.
    pub fn deploy<T: Tokenize>(
        &mut self,
        contract: SimulationContract<NotDeployed>,
        args: T,
    ) -> SimulationContract<IsDeployed> {
        // Append constructor args (if available) to generate the deploy bytecode;
        let constructor = contract.base_contract.abi().constructor();
        let bytecode = match constructor {
            Some(constructor) => constructor
                .encode_input(contract.bytecode.clone(), &args.into_tokens())
                .unwrap(),
            None => contract.bytecode.clone(),
        };

        // Take the execution result and extract the contract address.
        let execution_result = self.execute(
            self.admin.address,
            bytecode,
            TransactTo::create(),
            Uint::from(0),
        );
        let output = match execution_result {
            ExecutionResult::Success { output, .. } => output,
            ExecutionResult::Revert { output, .. } => panic!("Failed due to revert: {:?}", output),
            ExecutionResult::Halt { reason, .. } => panic!("Failed due to halt: {:?}", reason),
        };
        let contract_address = match output {
            Output::Create(_, address) => address.unwrap(),
            _ => panic!("failed"),
        };

        contract.to_deployed(contract_address)
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

        account.info.balance = amount;
    }
}

/// Recast a B160 into an Address type (perhaps this should be in utils?)
pub fn recast_address(address: B160) -> Address {
    let temp: [u8;20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}