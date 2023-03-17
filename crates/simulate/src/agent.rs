use std::str::FromStr;

use bytes::Bytes;
use ethers::{abi::Tokenize, prelude::BaseContract};
use revm::primitives::{
    Account, AccountInfo, ExecutionResult, Output, TransactTo, TxEnv, B160, U256,
};

use crate::environment::SimulationEnvironment;

pub struct TransactSettings {
    pub gas_limit: u64,
    pub gas_price: U256,
    pub value: U256,
}

pub struct SimulationManager {
    pub address: B160, //TODO: Consider using Address=H160 instead of B160
    pub account: Account,
    pub environment: SimulationEnvironment,
    pub transact_settings: TransactSettings,
}

pub trait Agent {
    fn transact(&mut self, tx: TxEnv) -> ExecutionResult;
    fn storage(&self) -> U256;
    fn build_call_transaction(&self, address: B160, call_data: Bytes, value: U256) -> TxEnv;
    // TODO: Should agents be labeled as `active` or `inactive` similarly to `IsDeployed` and `NotDeployed`?
}

impl Agent for SimulationManager {
    fn transact(&mut self, tx: TxEnv) -> ExecutionResult {
        self.environment.execute(tx)
    }
    fn storage(&self) -> U256 {
        todo!()
    }
    fn build_call_transaction(&self, address: B160, call_data: Bytes, value: U256) -> TxEnv {
        TxEnv {
            caller: self.address,
            gas_limit: self.transact_settings.gas_limit,
            gas_price: self.transact_settings.gas_price,
            gas_priority_fee: None,
            transact_to: TransactTo::Call(address),
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

    pub fn mint(&mut self, _address: B160, _amount: U256) {
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
        let execution_result = self.transact(deploy_transaction);
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
