use std::{collections::HashMap, str::FromStr};

use bytes::Bytes;
use ethers::{
    abi::Tokenize,
    prelude::{Address, BaseContract},
};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{
        Account, AccountInfo, ExecutionResult, Log, Output, TransactTo, TxEnv, B160, U256,
    },
    EVM,
};

use crate::agent::{Agent, TransactSettings};

struct SimulationEnvironment {
    evm: EVM<CacheDB<EmptyDB>>,
    _agents: HashMap<String, Box<dyn Agent>>,
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
            _agents: HashMap::new(), // This will only store agents that aren't the manager.
        }
    }
}

impl SimulationEnvironment {
    fn execute(&mut self, tx: TxEnv) -> ExecutionResult {
        self.evm.env.tx = tx;
        match self.evm.transact_commit() {
            Ok(val) => val,
            // URGENT: change this to a custom error
            Err(_) => panic!("failed"),
        }
    }
}

pub struct SimulationManager {
    pub address: B160, //TODO: Consider using Address=H160 instead of B160
    pub account: Account,
    environment: SimulationEnvironment,
    pub transact_settings: TransactSettings,
}

impl Agent for SimulationManager {
    fn call(&mut self, receiver_address: B160, call_data: Bytes, value: U256) -> ExecutionResult {
        let tx = self.build_call_transaction(receiver_address, call_data, value);
        self.environment.execute(tx)
    }
    fn get_logs(&mut self) -> &Vec<Log> {
        &self.environment.evm.db().unwrap().logs
    }

    fn build_call_transaction(
        &self,
        receiver_address: B160,
        call_data: Bytes,
        value: U256,
    ) -> TxEnv {
        TxEnv {
            caller: self.address,
            gas_limit: self.transact_settings.gas_limit,
            gas_price: self.transact_settings.gas_price,
            gas_priority_fee: None,
            transact_to: TransactTo::Call(receiver_address),
            value,
            data: call_data,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }
}
impl Default for SimulationManager {
    fn default() -> Self {
        Self {
            address: B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            account: Account::from(AccountInfo::default()),
            environment: SimulationEnvironment::default(),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO,
                value: U256::ZERO,
            },
        }
    }
}

impl SimulationManager {
    /// Used in the deploy function to create a transaction environment for deploying a contract.
    fn build_deploy_transaction(&self, bytecode: Bytes) -> TxEnv {
        TxEnv {
            caller: self.address,
            gas_limit: self.transact_settings.gas_limit,
            gas_price: self.transact_settings.gas_price,
            gas_priority_fee: None,
            transact_to: TransactTo::create(),
            value: U256::ZERO,
            data: bytecode,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }

    /// Deploy a contract. We will assume the sender is always the admin.
    /// TODO: This should call `recast_address` when a B160 is passed as an arg. Not sure how to handle this yet.
    pub fn deploy<T: Tokenize>(
        &mut self,
        contract: SimulationContract<NotDeployed>,
        args: T,
    ) -> SimulationContract<IsDeployed> {
        // Append constructor args (if available) to generate the deploy bytecode;
        let constructor = contract.base_contract.abi().constructor();
        let bytecode = match constructor {
            Some(constructor) => Bytes::from(
                constructor
                    .encode_input(contract.bytecode.clone(), &args.into_tokens())
                    .unwrap(),
            ),
            None => Bytes::from(contract.bytecode.clone()),
        };

        // Take the execution result and extract the contract address.
        // Manager address will always be the sender for contract deployments.

        let deploy_transaction = self.build_deploy_transaction(bytecode);
        let execution_result = self.environment.execute(deploy_transaction);
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

    // might be more appropriate in the manager
    /// Create a user account.
    pub fn create_user(&mut self, address: B160) {
        self.environment
            .evm
            .db()
            .unwrap()
            .insert_account_info(address, AccountInfo::default());
    }

    // might be more appropriate in the manager
    /// Give an address a specified amount of raw ether.
    fn _deal(&mut self, address: B160, amount: U256) {
        let account = self
            .environment
            .evm
            .db()
            .unwrap()
            .load_account(address)
            .unwrap();

        account.info.balance += amount;
    }

    pub fn mint(&mut self, _address: B160, _amount: U256) {
        todo!()
    }
}

/// Recast a B160 into an Address type (perhaps this should be in utils?)
pub fn recast_address(address: B160) -> Address {
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}

#[derive(Debug)]
pub struct NotDeployed;
#[derive(Debug)]
pub struct IsDeployed;

#[derive(Debug)]
pub struct SimulationContract<Deployed> {
    pub base_contract: BaseContract,
    pub bytecode: Vec<u8>,
    pub address: Option<B160>, /* TODO: Options may not be the best thing here. Also, B160 might not and Address=H160 might be. */
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
