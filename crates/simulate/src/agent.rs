use crate::environment::{self, *};
use ethers::{abi::Tokenize, prelude::BaseContract};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{
        ruint::Uint, Account, AccountInfo, ExecutionResult, Output, TransactTo, TxEnv, B160, U256,
    },
    EVM,
};
use std::str::FromStr;

pub struct SimulationManager {
    pub address: B160, //TODO: Consider using Address=H160 instead of B160
    pub account: Account,
    pub environment: SimulationEnvironment,
}

pub trait Agent {
    fn call(&self, environment: SimulationEnvironment, tx: TxEnv) -> ExecutionResult;
    fn storage(&self) -> U256;
    // TODO: Should agents be labeled as `active` or `inactive` similarly to `IsDeployed` and `NotDeployed`?
}

impl Agent for SimulationManager {
    fn call(&self, environment: SimulationEnvironment, tx: TxEnv) -> ExecutionResult {
        todo!()
    }
    fn storage(&self) -> U256 {
        todo!()
    }
}

impl SimulationManager {
    pub fn new() -> Self {
        Self {
            address: B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            account: Account::from(AccountInfo::default()),
            environment: SimulationEnvironment::new(),
        }
    }

    pub fn mint(&mut self, address: B160, amount: U256) {
        todo!()
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
            Some(constructor) => constructor
                .encode_input(contract.bytecode.clone(), &args.into_tokens())
                .unwrap(),
            None => contract.bytecode.clone(),
        };

        // Take the execution result and extract the contract address.
        // Manager address will always be the sender for contract deployments.
        let execution_result = self.environment.execute(
            self.address,
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
}

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