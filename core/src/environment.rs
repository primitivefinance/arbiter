#![warn(missing_docs)]
#![warn(unsafe_code)]

use crossbeam_channel::{unbounded, Receiver, Sender};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, U256},
    EVM,
};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
    thread,
};

use crate::utils::revm_logs_to_ethers_logs;
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
    pub connection: Connection,
    /// Clients (Agents) in the environment
    pub clients: Vec<Arc<Agent<RevmMiddleware>>>,
    // pub deployed_contracts: HashMap<String, Contract<RevmMiddleware>>,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub(crate) tx_sender: TxEnvSender,
    tx_receiver: TxEnvReceiver,
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
}

impl Environment {
    pub(crate) fn new(label: String) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.env.block.gas_limit = U256::MAX;
        let transaction_channel = unbounded::<(ToTransact, TxEnv, Sender<ExecutionResult>)>();
        let connection = Connection {
            tx_sender: transaction_channel.0,
            tx_receiver: transaction_channel.1,
            event_broadcaster: Arc::new(Mutex::new(EventBroadcaster::new())),
        };
        Self {
            label,
            state: State::Stopped,
            evm,
            connection,
            clients: vec![],
        }
    }
    // TODO: We need to make this the way to add agents to the environment.
    // in `agent.rs` we have `new_simulation_agent` which should probably just be called from this function instead.
    // OR agents can be created (without a connection?) and then added to the environment where they will gain a connection?
    pub fn add_agent(&mut self, agent: Agent<RevmMiddleware>) {
        self.clients.push(Arc::new(agent));
    }

    // TODO: Run should now run the agents as well as the evm.
    pub(crate) fn run(&mut self) {
        let tx_receiver = self.connection.tx_receiver.clone();
        let mut evm = self.evm.clone();
        let event_broadcaster = self.connection.event_broadcaster.clone();
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
