#![warn(missing_docs)]
#![warn(unsafe_code)]
//! ## module for the environment
//!
//! An abstraction on the EVM, to be used in simulations.
pub mod middleware;

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::{
    providers::{MockProvider, Provider}, signers::Signer,
};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, U256},
    EVM,
};
use std::{
    fmt,
    sync::{Arc, Mutex},
    thread, collections::HashMap,
};

use crate::agent::{Agent, AgentMiddleware};
use ethers::contract::Contract;
/// Type Aliases for the event channel.
pub(crate) type ExecutionSender = Sender<ExecutionResult>;
pub(crate) type TxEnvSender = Sender<(TxEnv, ExecutionSender)>;
pub(crate) type TxEnvReceiver = Receiver<(TxEnv, ExecutionSender)>;

/// The simulation environment that houses the execution environment and event logs.
/// # Fields
/// * `evm` - The EVM that is used for the simulation.
/// * `event_senders` - The senders on the event channel that is used to send events to the agents and simulation manager.
pub struct SimulationEnvironment<S, D, B> where S: Signer {
    /// The EVM that is used for the simulation.
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    /// The sender on the event channel that is used to send events to the agents and simulation manager.
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
    /// The receiver of txs from agents.
    /// Bundles with a sender to send the execution result back to the agent.
    pub(crate) transaction_channel: (TxEnvSender, TxEnvReceiver),
    /// The provider for [`Middleware`].
    pub(crate) provider: Provider<MockProvider>,
    /// Agents in the environment
    pub agents: Vec<Agent<S, D, B>>,
    /// The collection of different [`SimulationContract`] that are currently deployed in the [`SimulationEnvironment`].
    pub deployed_contracts: HashMap<String, Contract<AgentMiddleware<S>>>,
}

impl fmt::Debug for SimulationEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimulationEnvironment")
            // .field("evm", &self.evm)  // Skip the `evm` field
            .field("event_broadcaster", &self.event_broadcaster)
            .field("transaction_channel", &self.transaction_channel)
            .finish()
    }
}

impl SimulationEnvironment {
    pub(crate) fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.env.block.gas_limit = U256::MAX;
        evm.database(db);
        let transaction_channel = unbounded::<(TxEnv, Sender<ExecutionResult>)>();
        let provider = Provider::new(MockProvider::new());
        Self {
            evm,
            event_broadcaster: Arc::new(Mutex::new(EventBroadcaster::new())),
            transaction_channel,
            provider,
            agents: vec![],
            deployed_contracts: HashMap::new(),
        }
    }

    pub(crate) fn run(&self) {
        let tx_receiver = self.transaction_channel.1.clone();
        let mut evm = self.evm.clone();
        let event_broadcaster = self.event_broadcaster.clone();
        thread::spawn(move || {
            while let Ok((tx, sender)) = tx_receiver.recv() {
                // Execute the transaction, echo the logs to all agents, and report the execution result to the agent who made the call.
                let execution_result = execute(&mut evm, tx);
                event_broadcaster
                    .lock()
                    .unwrap()
                    .broadcast(execution_result.logs());
                sender.send(execution_result).unwrap();
            }
        });
    }
}

/// Execute a transaction in the execution environment.
/// # Arguments
/// * `tx` - The transaction environment that is used to execute the transaction.
/// # Returns
/// * `ExecutionResult` - The execution result of the transaction.
fn execute(evm: &mut EVM<CacheDB<EmptyDB>>, tx: TxEnv) -> ExecutionResult {
    evm.env.tx = tx;

    match evm.transact_commit() {
        Ok(val) => val,
        // URGENT: change this to a custom error
        Err(_) => panic!("failed"),
    }
}

/// The event broadcaster that is used to broadcast events to the agents from the simulation manager.
#[derive(Clone, Debug)]
pub struct EventBroadcaster(Vec<crossbeam_channel::Sender<Vec<Log>>>);

impl EventBroadcaster {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }

    pub(crate) fn add_sender(&mut self, sender: crossbeam_channel::Sender<Vec<Log>>) {
        self.0.push(sender);
    }

    pub(crate) fn broadcast(&self, logs: Vec<Log>) {
        for sender in &self.0 {
            sender.send(logs.clone()).unwrap();
        }
    }
}
