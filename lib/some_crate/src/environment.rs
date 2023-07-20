#![warn(missing_docs)]
#![warn(unsafe_code)]

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::middleware::providers::RpcError;
use ethers::providers::{JsonRpcClient, MiddlewareError};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, U256},
    EVM,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex},
    thread,
};
use thiserror::Error;

use crate::{agent::Agent, middleware::RevmMiddleware};
use ethers::contract::Contract;
/// Type Aliases for the event channel.
pub(crate) type ExecutionSender = Sender<ExecutionResult>;
pub(crate) type TxEnvSender = Sender<(TxEnv, ExecutionSender)>;
pub(crate) type TxEnvReceiver = Receiver<(TxEnv, ExecutionSender)>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum State {
    /// The [`Environment`] is currently running.
    /// [`Agent`]s cannot be added if the environment is [`State::Running`].
    Running,
    /// The [`Environment`] is currently stopped.
    /// [`Agent`]s can only be added if the environment is [`State::Stopped`].
    Stopped,
}

// TODO: RENAME PROVIDER HERE IT IS BAD
pub struct Environment {
    pub label: String,
    pub(crate) state: State,
    pub(crate) provider: RevmProvider, //bad name lol
    /// Clients (Agents) in the environment
    pub clients: Vec<Arc<Agent<RevmMiddleware>>>,

    pub deployed_contracts: HashMap<String, Contract<RevmMiddleware>>,
}

// TODO: Implement the request function for the provider here so that we can use the
// get_transaction call
pub(crate) struct RevmProvider {
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    pub(crate) connection: Connection,
}

impl Debug for RevmProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RevmProvider")
            .field("EVM<CacheDB>", &self.evm.db)
            .field("connection", &self.connection)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub(crate) tx_sender: TxEnvSender,
    tx_receiver: TxEnvReceiver,
    event_broadcaster: Arc<Mutex<EventBroadcaster>>,
}

#[async_trait::async_trait]
impl JsonRpcClient for Connection {
    #[doc = " A JSON-RPC Error"]
    type Error = RevmProviderError;

    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        match method {
            "eth_getTransactionByHash" => {
                // Ok(ethers::core::types::Transaction::default())
                Ok(serde_json::from_str("thing").unwrap())
            }
            _ => {
                panic!("Method not implemented")
            }
        }
    }
}

impl Environment {
    pub(crate) fn new(label: String) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.env.block.gas_limit = U256::MAX;
        let transaction_channel = unbounded::<(TxEnv, Sender<ExecutionResult>)>();
        let connection = Connection {
            tx_sender: transaction_channel.0,
            tx_receiver: transaction_channel.1,
            event_broadcaster: Arc::new(Mutex::new(EventBroadcaster::new())),
        };
        let provider = RevmProvider { evm, connection };
        Self {
            label,
            state: State::Stopped,
            provider,
            clients: vec![],
            deployed_contracts: HashMap::new(),
        }
    }

    pub fn add_agent(&mut self, agent: Agent<RevmMiddleware>) {
        self.clients.push(Arc::new(agent));
    }

    // TODO: Run should now run the agents as well as the evm.
    pub(crate) fn run(&mut self) {
        let tx_receiver = self.provider.connection.tx_receiver.clone();
        let mut evm = self.provider.evm.clone();
        let event_broadcaster = self.provider.connection.event_broadcaster.clone();
        self.state = State::Running;

        //give all agents their own thread and let them start watching for their evnts
        thread::spawn(move || {
            while let Ok((tx, sender)) = tx_receiver.recv() {
                // Execute the transaction, echo the logs to all agents, and report the execution result to the agent who made the call.
                evm.env.tx = tx;
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
                let db = evm.db().unwrap();
            }
        });
    }
}

#[derive(Debug, Error)]
pub enum RevmProviderError {
    #[error("The `RevmEnvironment` had an error.")]
    Error,
}

impl RpcError for RevmProviderError {
    fn as_error_response(&self) -> Option<&ethers::providers::JsonRpcError> {
        todo!()
    }

    fn as_serde_error(&self) -> Option<&serde_json::Error> {
        todo!()
    }
}

impl From<RevmProviderError> for ethers::providers::ProviderError {
    fn from(err: RevmProviderError) -> Self {
        ethers::providers::ProviderError::CustomError(err.to_string())
    }
}

impl MiddlewareError for RevmProviderError {
    type Inner = Self;

    fn from_err(e: Self::Inner) -> Self {
        todo!()
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        todo!()
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::bindings::writer::Writer;
    use anyhow::Result;

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
