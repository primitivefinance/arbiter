use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
};

use bytes::Bytes;
use ethers::{
    abi::Tokenize,
    prelude::{Address, BaseContract},
};
use crate::environment::IsDeployed;
use crate::environment::NotDeployed;
use crate::environment::SimulationContract;
use crate::environment::SimulationEnvironment;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{
        Account, AccountInfo, ExecutionResult, Log, Output, TransactTo, TxEnv, B160, U256,
    },
    EVM,
};

use crate::agent::{Agent, TransactSettings};

pub struct SimulationManager<'a> {
    /// Public address of the simulation manager.
    pub address: B160, //TODO: Consider using Address=H160 instead of B160, or wait for ruints.
    /// revm-primitive account of the simulation manager.
    pub account: Account,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    pub transact_settings: TransactSettings,

    /// `SimulationEnvironment` that the simulation manager controls.
    environment: SimulationEnvironment<'a>,
}

pub fn recast_address(address: B160) -> Address {
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}

impl<'a> SimulationManager<'a> {
    /// Constructor function to instantiate a 
    pub fn new() -> Self {
        Self {
            address: B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            account: Account::from(AccountInfo::default()),
            environment: SimulationEnvironment::new(),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO,
            },
        }
    }

    /// Run all agents concurrently in the current simulation environment.
    pub fn run_agents() {
        todo!()
    }

    /// Add an [`Agent`] to the current simulation.
    pub fn add_agent(&mut self, name: &'a str, agent: Box<dyn Agent>) {
        self.environment.agents.insert(name, agent).unwrap();
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

    fn call_contract(
        &mut self,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
    ) -> ExecutionResult {
        let tx = self.build_transaction(contract.address.unwrap(), call_data, value);
        self.environment.execute(tx)
    }

    // TODO: Handle the output of the execution result and decode?
    /// Gets the most current event (which is all that is stored in the event buffer).
    fn read_logs(&mut self) -> Vec<Log> {
        self.environment.event_buffer.read().unwrap().to_vec()
    }

    /// Build a `TxEnv` which the EVM uses natively.
    fn build_transaction(
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

    /// Takes an `ExecutionResult` and returns the raw bytes of the output that can then be decoded.
    pub fn unpack_execution(&self, execution_result: ExecutionResult) -> Bytes {
        match execution_result {
            ExecutionResult::Success { output, .. } => match output {
                Output::Call(value) => value,
                Output::Create(_, Some(_)) => {
                    panic!("Failed. This was a 'Create' call, use 'Deploy' instead.")
                }
                _ => panic!("This call has failed."),
            },
            _ => panic!("This call generated no execution result. This should not happen."),
        }
    }
}