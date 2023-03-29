#![warn(missing_docs)]
use std::cell::RefMut;
use std::str::FromStr;
use std::{cell::RefCell, rc::Rc};
use bytes::Bytes;

use ethers::abi::Tokenize;
use revm::primitives::{
    Account, AccountInfo, Address, ExecutionResult, Log, Output, TransactTo, TxEnv, B160, U256,
};

use crate::{
    agent::{Agent, TransactSettings},
    environment::{IsDeployed, NotDeployed, SimulationContract, SimulationEnvironment},
};

pub struct Admin {
    /// Public address of the simulation manager.
    pub address: B160,
    /// revm-primitive account of the simulation manager.
    pub account: Account,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    transact_settings: TransactSettings,
    // TODO: is this useful? environment: Arc<Mutex<Environment>>,
    environment: Rc<RefCell<SimulationEnvironment>>,
}

impl Agent for Admin {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn simulation_environment(&self) -> RefMut<SimulationEnvironment> {
        self.environment.borrow_mut()
    }
}

impl Admin {
    /// Constructor function to instantiate a
    pub fn new(environment: &Rc<RefCell<SimulationEnvironment>>) -> Self {
        Self {
            address: B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            account: Account::from(AccountInfo::default()),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO,
            },
            environment: environment.clone(),
        }
    }
    /// Deploy a contract to the current simulation environment.
    pub fn deploy<T: Tokenize>(
        &mut self,
        contract: SimulationContract<NotDeployed>,
        args: T,
    ) -> SimulationContract<IsDeployed> {
        // Append constructor args (if available) to generate the deploy bytecode.
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
        let execution_result = self.simulation_environment().execute(deploy_transaction);
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
}
