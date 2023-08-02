#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    collections::HashMap,
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::core::types::U64;
use ethers::providers::{JsonRpcClient, ProviderError};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, U256},
    EVM,
};

use crate::{
    agent::Agent,
    math::stochastic_process::poisson_process,
    middleware::RevmMiddleware,
    utils::{convert_uint_to_u64, revm_logs_to_ethers_logs},
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
    thread,
};

use crate::{utils::revm_logs_to_ethers_logs, agent::IsAttached};
use crate::{agent::Agent, middleware::RevmMiddleware};

/// Type Aliases for the event channel.
pub(crate) type ToTransact = bool;
pub(crate) type ExecutionSender = Sender<RevmResult>;
pub(crate) type TxEnvSender = Sender<(ToTransact, TxEnv, ExecutionSender)>;
pub(crate) type TxEnvReceiver = Receiver<(ToTransact, TxEnv, ExecutionSender)>;

/// Result struct for the [`Environment`]. that wraps the [`ExecutionResult`] and the block number.
#[derive(Debug, Clone)]
pub struct RevmResult {
    pub(crate) result: ExecutionResult,
    pub(crate) block_number: U64,
}

/// State enum for the [`Environment`].
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum State {
    /// The [`Environment`] is currently running.
    /// [`Agent`]s cannot be added if the environment is [`State::Running`].
    Running,
    /// The [`Environment`] is currently stopped.
    /// [`Agent`]s can only be added if the environment is [`State::Stopped`].
    Stopped,
}

/// The environment struct.
pub struct Environment {
    /// label for the environment
    pub label: String,
    pub(crate) state: State,
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    /// Connection to the environment
    pub connection: Connection,
    /// Clients (Agents) in the environment
    pub clients: HashMap<String, Agent<RevmMiddleware>>,
    /// expected events per block
    pub lambda: Some(f64),
}

/// Connection struct for the [`Environment`].
#[derive(Debug, Clone)]
pub struct Connection {
    pub(crate) tx_sender: TxEnvSender,
    tx_receiver: TxEnvReceiver,
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
    /// expected events per block
    pub tx_per_block: Arc<AtomicUsize>,
}

impl Environment {
    /// Create a new [`Environment`].
    pub(crate) fn new(label: String) -> Self {
    pub(crate) tx_sender: TxEnvSender,
    tx_receiver: TxEnvReceiver,
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
    /// Clients (Agents) in the environment
    pub agents: Vec<Agent<IsAttached<RevmMiddleware>>>,
    // pub deployed_contracts: HashMap<String, Contract<RevmMiddleware>>,
}

// TODO: If the provider holds the connection then this can work better.
pub struct RevmProvider {
    pub(crate) tx_sender: TxEnvSender,
    pub(crate) result_sender: crossbeam_channel::Sender<ExecutionResult>,
    pub(crate) result_receiver: crossbeam_channel::Receiver<ExecutionResult>,
    pub(crate) event_receiver: crossbeam_channel::Receiver<Vec<ethers::types::Log>>,
}

impl Debug for RevmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RevmProvider").finish()
    }
}

#[async_trait::async_trait]
impl JsonRpcClient for RevmProvider {
    type Error = ProviderError;

    async fn request<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ProviderError> {
        match method {
            "eth_getFilterChanges" => {
                let logs = self.event_receiver.recv().unwrap();
                println!("logs: {:?}", logs);
                let logs_str = serde_json::to_string(&logs).unwrap();
                let logs_deserializeowned: R = serde_json::from_str(&logs_str)?;
                return Ok(logs_deserializeowned);
                // return Ok(serde::to_value(self.event_receiver.recv().ok()).unwrap())
            },
            _ => {
                unimplemented!("We don't cover this case yet.")
            }
        }
    }
}

impl Environment {
    pub(crate) fn new<S: Into<String>>(label: S) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.env.block.gas_limit = U256::MAX;
        let (tx_sender, tx_receiver) = unbounded::<(ToTransact, TxEnv, Sender<RevmResult>)>();
        let connection = Connection {
            tx_sender: tx_sender,
            tx_receiver: tx_receiver,
            event_broadcaster: Arc::new(Mutex::new(EventBroadcaster::new())),
            tx_per_block: Arc::new(AtomicUsize::new(0)),
        };
        Self {
            label: label.into(),
            state: State::Stopped,
            evm,
            connection,
            clients: HashMap::new(),
            // Default is none
            lambda: None,
        }
    }

    pub(crate) fn configure_lambda(&mut self, lamda: f64) {
        self.lambda = Some(lamda);
    }

    /// Creates a new [`Agent<RevmMiddleware`] with the given label.
    pub(crate) fn add_agent(&mut self, name: String) {
        let agent = Agent::new_simulation_agent(name.clone(), &self.connection);
        self.clients.insert(name, agent);
            tx_sender,
            tx_receiver,
            event_broadcaster: Arc::new(Mutex::new(EventBroadcaster::new())),
            agents: vec![],
        }
    }
    // TODO: We need to make this the way to add agents to the environment.
    // in `agent.rs` we have `new_simulation_agent` which should probably just be called from this function instead.
    // OR agents can be created (without a connection?) and then added to the environment where they will gain a connection?
    pub fn add_agent(&mut self, agent: Agent<IsAttached<RevmMiddleware>>) {
        self.agents.push(agent);
    }

    // TODO: Run should now run the agents as well as the evm.
    pub(crate) fn run(&mut self) {
        let tx_receiver = self.tx_receiver.clone();
        let mut evm = self.evm.clone();
        let event_broadcaster = self.connection.event_broadcaster.clone();
        let counter = Arc::clone(&self.connection.tx_per_block);
        let expected_occurance = poisson_process(self.lambda).unwrap();
        self.state = State::Running;

        //give all agents their own thread and let them start watching for their evnts
        thread::spawn(move || {
            while let Ok((to_transact, tx, sender)) = tx_receiver.recv() {
                // Execute the transaction, echo the logs to all agents, and report the execution result to the agent who made the transaction.
                if counter.load(Ordering::Relaxed) >= expected_occurance[0] as usize {
                    evm.env.block.number += U256::from(1);
                    counter.store(0, Ordering::Relaxed);
                }
                evm.env.tx = tx;
                if to_transact {
                    let execution_result = match evm.transact_commit() {
                        Ok(val) => val,
                        // URGENT: change this to a custom error
                        Err(_) => panic!("failed"),
                    };
                    event_broadcaster
                        .lock()
                        .unwrap()
                        .broadcast(execution_result.logs());
                    let execution_result = RevmResult {
                        result: execution_result,
                        block_number: convert_uint_to_u64(evm.env.block.number).unwrap(),
                    };
                    sender.send(execution_result).unwrap();
                    counter.fetch_add(1, Ordering::Relaxed);
                } else {
                    let execution_result = match evm.transact() {
                        Ok(val) => val,
                        // URGENT: change this to a custom error
                        Err(_) => panic!("failed"),
                    };
                    let result_and_block = RevmResult {
                        result: execution_result.result,
                        block_number: convert_uint_to_u64(evm.env.block.number).unwrap(),
                    };
                    sender.send(result_and_block).unwrap();
                }
            }
        });
    }
}

/// The event broadcaster that is used to broadcast events to the agents from the simulation manager.
#[derive(Clone, Debug)]
pub struct EventBroadcaster(Vec<crossbeam_channel::Sender<Vec<ethers::core::types::Log>>>);

impl EventBroadcaster {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }

    pub(crate) fn add_sender(
        &mut self,
        sender: crossbeam_channel::Sender<Vec<ethers::core::types::Log>>,
    ) {
        self.0.push(sender);
    }

    pub(crate) fn broadcast(&self, logs: Vec<Log>) {
        for sender in &self.0 {
            sender.send(revm_logs_to_ethers_logs(logs.clone())).unwrap();
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) const TEST_ENV_LABEL: &str = "test";

    #[test]
    fn new() {
        let env = Environment::new(TEST_ENV_LABEL.to_string());
        assert_eq!(env.label, TEST_ENV_LABEL);
        assert_eq!(env.state, State::Stopped);
    }

    #[test]
    fn run() {
        let mut environment = Environment::new(TEST_ENV_LABEL.to_string());
        environment.run();
        assert_eq!(environment.state, State::Running);
    }
}
