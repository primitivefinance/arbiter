#![warn(missing_docs)]
//! This module contains the structs necessary to serve as an environment that a simulation that can be run inside of.
//! The `SimulationManager` inherits the `Agent` trait and is given control over the `SimulationEnvironment` struct.
//! The job of the manager is to deploy contracts and call upon special functionality that guides the simulation forward.

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
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{
        Account, AccountInfo, ExecutionResult, Log, Output, TransactTo, TxEnv, B160, U256,
    },
    EVM,
};

use crate::agent::{Agent, TransactSettings};

/// Controls the execution environment (EVM), has an associated set of agents, and provides an event (ETH log) buffer.
struct SimulationEnvironment {
    _agents: HashMap<String, Box<dyn Agent>>,
    event_buffer: Arc<RwLock<Vec<Log>>>, // TODO: This should probably just store head
    writer_thread: Option<thread::JoinHandle<()>>,

    evm: EVM<CacheDB<EmptyDB>>,
}

impl SimulationEnvironment {
    /// Private and can only be called by the `SimulationManager` struct.
    /// This prevents a user from creating an `ExecutionManager` without a `SimulationManager` which will not function well.
    fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.database(db);

        Self {
            evm,
            event_buffer: Arc::new(RwLock::new(Vec::<Log>::new())),
            writer_thread: Some(thread::spawn(|| {})),
            _agents: HashMap::new(), // This will only store agents that aren't the manager.
        }
    }
}

/// `SimulationEnvironment` serves as a mechanism for getting EVM computation results and distributing logs to agents.
impl SimulationEnvironment {
    /// `execute` is a public function that takes a transaction and executes it on the EVM.
    fn execute(&mut self, tx: TxEnv) -> ExecutionResult {
        self.evm.env.tx = tx;
        let execution_result = match self.evm.transact_commit() {
            Ok(val) => val,
            // URGENT: change this to a custom error
            Err(_) => panic!("failed"),
        };

        let logs = execution_result.logs();
        self.echo_logs(logs);
        execution_result
    }
    
    /// `echo_logs` is a private function that takes a vector of logs and writes them to the event buffer that all agents can read.
    fn echo_logs(&mut self, logs: Vec<Log>) {
        // let writer_thread = self.writer_thread.take();
        if let Some(handle) = self.writer_thread.take() {
            handle.join().unwrap();
        }
        self.event_buffer.write().unwrap().clear();
        logs.into_iter().for_each(|log| {
            self.event_buffer.write().unwrap().push(log);
        });
    }
    // TODO: Implementing the following functions could be useful.
    // fn decode_event;
    // fn decode_output; // These two could be combined.
    // fn unpack_execution_result;
}
/**
`SimulationManager` is the main struct that is used to control the simulation environment.
It acts in some ways as an agent, but it also has special control over the environment such as deploying contracts.
It is the only agent that contains an execution environment.
*/
pub struct SimulationManager {
    /// `address` is the public address of the simulation manager.
    pub address: B160, //TODO: Consider using Address=H160 instead of B160, or wait for ruints.
    /// `account` is the revm-primitive account of the simulation manager.
    pub account: Account,
    /// `environment` is the `SimulationEnvironment` that the simulation manager controls.
    environment: SimulationEnvironment,
    /// `transact_settings` is a struct that contains the default transaction options for revm such as gas limit and gas price.
    pub transact_settings: TransactSettings,
}

impl Agent for SimulationManager {
    // TODO: Can calling a contract ever need for raw eth "value" to be sent along with?
    /// `call_contract` is a public function that takes a contract call and passes it to the execution environment.
    fn call_contract(
        &mut self,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract.address.unwrap(), call_data, value);
        self.environment.execute(tx)
    }
    // TODO: Handle the output of the execution result and decode?
    /// `read_logs` gets the most current event (which is all that is stored in the event buffer).
    fn read_logs(&mut self) -> Vec<Log> {
        self.environment.event_buffer.read().unwrap().to_vec()
    }
    /// `build_call_transaction` is a constructor function that takes an address and an account and returns a `TxEnv` which the EVM uses natively.
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
    /// `default` is a public constructor function that returns a `SimulationManager` with default values.
    /// Unless more simulations are being run in parallel, this is the only constructor function that should be used since a manager comes equipped with an environment.
    fn default() -> Self {
        Self {
            address: B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            account: Account::from(AccountInfo::default()),
            environment: SimulationEnvironment::default(),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO,
            },
        }
    }
}

impl SimulationManager {
    // TODO: Unpacking should probably be defined on ExecutionResult type. But that will be in the crate.
    /// `unpack_execution` is a public function that takes an `ExecutionResult` and returns the raw bytes of the output that can then be decoded.
    pub fn unpack_execution(&self, execution_result: ExecutionResult) -> Bytes {
        // unpack output call enum into raw bytes
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
    /// `build_deploy_transaction` is a constructor function for `Create` types of transactions.
    /// This is special to the `SimulationManager` since it is the only agent that can deploy contracts.
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

    // TODO: This should call `recast_address` when a B160 is passed as an arg. Not sure how to handle this yet.
    /// Deploy a contract. We will assume the sender is always the admin.
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
    /// `mint` can be used to mint certain tokens to specific addresses.
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
/// A struct use for `PhantomData` to indicate a lock on contracts that are not deployed.
pub struct NotDeployed;
#[derive(Debug)]
// TODO: It would be good to also allow the `IsDeployed` to also mention which `SimulationManager` has deployed it (when we have multiple managers).
/// A struct use for `PhantomData` to indicate an unlocked contract that is deployed.
pub struct IsDeployed;

#[derive(Debug)]
/// A struct that wraps around the ethers `BaseContract` and adds some additional information relevant for revm and the simulation.
pub struct SimulationContract<Deployed> {
    /// The ethers `BaseContract` that holds the ABI.
    pub base_contract: BaseContract,
    // TODO: Bytecode is really only used for deployment. Maybe we don't need to store it like this.
    /// The contract's deployed bytecode.
    pub bytecode: Vec<u8>,
    //TODO: Options are not great here. We want an address for the deployment to some `SimulationEnvironment`.
    /// The address of the contract within the relevant `SimulationEnvironment`.
    pub address: Option<B160>,
    /// A `PhantomData` marker to indicate whether the contract is deployed or not.
    pub deployed: std::marker::PhantomData<Deployed>,
}

impl SimulationContract<NotDeployed> {
    /// A constructor function for `SimulationContract` that takes a `BaseContract` and the deployment bytecode.
    pub fn new(base_contract: BaseContract, bytecode: Vec<u8>) -> Self {
        Self {
            base_contract,
            bytecode,
            address: None,
            deployed: std::marker::PhantomData,
        }
    }
    /// Creates a new `SimulationContract` that is marked as deployed.
    /// This is only called by implicitly by the `SimulationManager` inside of the `deploy` function.
    fn to_deployed(&self, address: B160) -> SimulationContract<IsDeployed> {
        SimulationContract {
            base_contract: self.base_contract.clone(),
            bytecode: self.bytecode.clone(),
            address: Some(address),
            deployed: std::marker::PhantomData,
        }
    }
}
