#![warn(missing_docs)]
//! The environment that constitutes a simulation is handled here.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
};

use ethers::prelude::BaseContract;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, B160},
    EVM,
};

// use crate::agent::Agent;

pub struct SimulationEnvironment {
    /// The EVM that is used for the simulation.
    evm: EVM<CacheDB<EmptyDB>>,
    /// The buffer agents can read from.
    pub(crate) event_buffer: Arc<RwLock<Vec<Log>>>,
    /// Thread that is used to write to the event buffer.
    pub(crate) writer_thread: Option<thread::JoinHandle<()>>,
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

impl SimulationEnvironment {
    pub(crate) fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.database(db);

        Self {
            evm,
            event_buffer: Arc::new(RwLock::new(Vec::<Log>::new())),
            writer_thread: Some(thread::spawn(|| {})),
        }
    }

    pub(crate) fn execute(&mut self, tx: TxEnv) -> ExecutionResult {
        self.evm.env.tx = tx;

        let execution_result = match self.evm.transact_commit() {
            Ok(val) => val,
            // URGENT: change this to a custom error
            Err(_) => panic!("failed"),
        };

        self.echo_logs(execution_result.logs());

        execution_result
    }

    pub(crate) fn echo_logs(&mut self, logs: Vec<Log>) {
        if let Some(handle) = self.writer_thread.take() {
            handle.join().unwrap();
        }

        self.event_buffer.write().unwrap().clear();

        logs.into_iter().for_each(|log| {
            self.event_buffer.write().unwrap().push(log);
        });
    }
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
    pub fn to_deployed(&self, address: B160) -> SimulationContract<IsDeployed> {
        SimulationContract {
            base_contract: self.base_contract.clone(),
            bytecode: self.bytecode.clone(),
            address: Some(address),
            deployed: std::marker::PhantomData,
        }
    }
}
