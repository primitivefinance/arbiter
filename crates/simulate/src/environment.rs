#![warn(missing_docs)]
//! The environment that constitutes a simulation is handled here.

use std::thread;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::RwLock as AsyncRwLock;

use ethers::prelude::BaseContract;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, B160},
    EVM,
};

use futures::{stream::{self, Map, StreamExt}, channel::mpsc::UnboundedReceiver};
use futures::channel::mpsc::{UnboundedSender, unbounded};
use tokio::task::JoinHandle;

/// The simulation environment that houses the execution environment and event logs.
pub struct SimulationEnvironment {
    /// The EVM that is used for the simulation.
    pub(crate) evm: Arc<AsyncRwLock<EVM<CacheDB<EmptyDB>>>>,
    /// The buffer agents can read from.
    pub(crate) event_sender: UnboundedSender<Vec<Log>>,
    pub(crate) event_receiver: UnboundedReceiver<Vec<Log>>,
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
        let (event_sender, mut event_receiver) = unbounded();

        Self {
            evm: Arc::new(AsyncRwLock::new(evm)),
            // event_buffer: Vec::<Log>::new(),
            event_sender,
            event_receiver,
        }
    }

    pub(crate) async fn execute(&mut self, tx: TxEnv) -> ExecutionResult {
        self.evm.write().await.env.tx = tx;

        let execution_result = match self.evm.write().await.transact_commit() {
            Ok(val) => val,
            // URGENT: change this to a custom error
            Err(_) => panic!("failed"),
        };
        self.echo_logs(execution_result.logs());

        execution_result
    }
    pub(crate) fn echo_logs(&mut self, logs: Vec<Log>) {
        println!("echo_logs: {:?}", logs);
        self.event_sender.unbounded_send(logs).unwrap(); // TODO: Add error checking?
    }
    // pub(crate) fn echo_logs(&mut self, logs: Vec<Log>) {
    //     self.event_buffer.clear();

    //     logs.into_iter().for_each(|log| {
    //         self.event_buffer.push(log);
    //     });
    // }
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
