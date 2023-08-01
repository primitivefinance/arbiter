#![warn(missing_docs)]
#![warn(unsafe_code)]

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::{providers::{JsonRpcClient, ProviderError}, types::{Filter, H256}, prelude::k256::sha2::{Digest, Sha256}, utils::serialize};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, U256},
    EVM,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
    thread, collections::HashMap,
};

use crate::{utils::revm_logs_to_ethers_logs, agent::IsAttached};
use crate::{agent::Agent, middleware::RevmMiddleware};

/// Type Aliases for the event channel.
pub(crate) type ToTransact = bool;
pub(crate) type ExecutionSender = Sender<ExecutionResult>;
pub(crate) type TxEnvSender = Sender<(ToTransact, TxEnv, ExecutionSender)>;
pub(crate) type TxEnvReceiver = Receiver<(ToTransact, TxEnv, ExecutionSender)>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum State {
    /// The [`Environment`] is currently running.
    /// [`Agent`]s cannot be added if the environment is [`State::Running`].
    Running,
    /// The [`Environment`] is currently stopped.
    /// [`Agent`]s can only be added if the environment is [`State::Stopped`].
    Stopped,
}

pub struct Environment {
    pub label: String,
    pub(crate) state: State,
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
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
    // pub(crate) filter_receivers: HashMap<ethers::types::U256, crossbeam_channel::Receiver<Vec<ethers::types::Log>>>, // TODO: Use this to replace event_receivers so we can look for updates in specific filters
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
                // Store a Map of filters with their IDs as keys
                let logs = self.event_receiver.recv().unwrap();
                println!("logs: {:?}", logs);
                let logs_str = serde_json::to_string(&logs).unwrap();
                let logs_deserializeowned: R = serde_json::from_str(&logs_str)?;
                return Ok(logs_deserializeowned);
                // return Ok(serde::to_value(self.event_receiver.recv().ok()).unwrap())
            },
            "eth_newFilter" => {
                let filter_string = serde_json::to_string(&params).unwrap();
                let filter: Filter = serde_json::from_str(&filter_string).unwrap();
                let mut hasher = Sha256::new();
                hasher.update(filter_string.as_bytes());
                let hash = hasher.finalize();
                let id = ethers::types::U256::from(ethers::types::H256::from_slice(&hash).as_bytes());
                let id_str = serde_json::to_string(&id).unwrap();
                let id_deserializeowned: R = serde_json::from_str(&id_str)?;
                Ok(id_deserializeowned)
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
        let (tx_sender, tx_receiver) = unbounded::<(ToTransact, TxEnv, Sender<ExecutionResult>)>();
        Self {
            label: label.into(),
            state: State::Stopped,
            evm,
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
        let event_broadcaster = self.event_broadcaster.clone();
        self.state = State::Running;

        //give all agents their own thread and let them start watching for their evnts
        thread::spawn(move || {
            while let Ok((to_transact, tx, sender)) = tx_receiver.recv() {
                // Execute the transaction, echo the logs to all agents, and report the execution result to the agent who made the transaction.
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
                    sender.send(execution_result).unwrap();
                } else {
                    let execution_result = match evm.transact() {
                        Ok(val) => val,
                        // URGENT: change this to a custom error
                        Err(_) => panic!("failed"),
                    };
                    sender.send(execution_result.result).unwrap();
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
