#![warn(missing_docs)]
#![warn(unsafe_code)]

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::{
    providers::{MockProvider, Provider},
    types::{Address, BlockId},
    middleware::providers::{JsonRpcClient, RpcError},
};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, U256},
    EVM,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    fmt::Formatter,
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
    /// The [`SimulationEnvironment`] is currently running.
    /// [`Agent`]s cannot be added if the environment is [`State::Running`].
    Running,
    /// The [`SimulationEnvironment`] is currently stopped.
    /// [`Agent`]s can only be added if the environment is [`State::Stopped`].
    Stopped,
}

/// The simulation environment that houses the execution environment and event logs.
/// # Fields
/// * `evm` - The EVM that is used for the simulation.
/// * `event_senders` - The senders on the event channel that is used to send events to the agents and simulation manager.
pub struct RevmEnvironment {
    pub label: String,
    pub(crate) state: State,
    /// The EVM that is used for the simulation.
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    /// The sender on the event channel that is used to send events to the agents and simulation manager.
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
    /// The receiver of txs from agents.
    /// Bundles with a sender to send the execution result back to the agent.
    pub(crate) transaction_channel: (TxEnvSender, TxEnvReceiver),
    /// Agents in the environment
    pub agents: Vec<Agent>,

    pub deployed_contracts: HashMap<String, Contract<RevmMiddleware>>,
}

impl std::fmt::Debug for RevmEnvironment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RevmEnvironment")
            .field("label", &self.label)
            .field("state", &self.state)
            .field("event_broadcaster", &self.event_broadcaster)
            .field("transaction_channel", &self.transaction_channel)
            .field("agents", &self.agents)
            .field("deployed_contracts", &self.deployed_contracts)
            .finish()
    }
}

impl RevmEnvironment {
    pub(crate) fn new(label: String) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.env.block.gas_limit = U256::MAX;
        evm.database(db);
        let transaction_channel = unbounded::<(TxEnv, Sender<ExecutionResult>)>();
        let provider = Provider::new(MockProvider::new());
        Self {
            label,
            evm,
            event_broadcaster: Arc::new(Mutex::new(EventBroadcaster::new())),
            transaction_channel,
            agents: vec![],
            deployed_contracts: HashMap::new(),
            state: State::Stopped,
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent);
    }

    // TODO: Run should now run the agents as well as the evm.
    pub(crate) fn run(&mut self) {
        let tx_receiver = self.transaction_channel.1.clone();
        let mut evm = self.evm.clone();
        let event_broadcaster = self.event_broadcaster.clone();
        self.state = State::Running;
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
            }
        });
    }
}

#[derive(Debug, Error)]
pub enum RevmEnvironmentError {
    #[error("The `RevmEnvironment` had an error.")]
    Error,
}

impl RpcError for RevmEnvironmentError {
    fn as_error_response(&self) -> Option<&ethers_middleware::providers::JsonRpcError> {
        todo!()
    }

    fn as_serde_error(&self) -> Option<&serde_json::Error> {
        todo!()
    }
}

impl From<RevmEnvironmentError> for ethers::providers::ProviderError {
    fn from(err: RevmEnvironmentError) -> Self {
        ethers::providers::ProviderError::CustomError(err.to_string())
    }
}

// TODO: This request function has exposed that we need to be careful of how we store data of blocks.
// I'm not sure exactly how we should do this. 
// We can change the BlockEnv, but I don't know all the effects of this. 
// If we want to get the historical balance of an account, we need to store the block number that the balance was at.
// We could potentially save the state of the DB after each block.
#[async_trait::async_trait]
impl JsonRpcClient for RevmEnvironment {
    type Error = RevmEnvironmentError;
    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Self::Error>
    where
        T: std::fmt::Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        match method {
            "eth_getBalance" => {
                let (address, _block_id) = serde_json::from_value::<(Address, Option<BlockId>)>(params)?;
                let accounts = self.evm.db().unwrap().accounts;
                let account = accounts.get(&address).unwrap();
                let balance = account.balance;
                Ok(balance)
            }
        }
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
mod tests {
    use super::*;
    use ethers::providers::{Middleware, Provider};

    #[test]
    fn new() {
        let env = RevmEnvironment::new("test".to_string());
        assert_eq!(env.label, "test");
        assert_eq!(env.state, State::Stopped);
        // assert_eq!(env.agents, vec![]);
        // assert_eq!(env.deployed_contracts, HashMap::new());
    }

    #[test]
    fn run() {}

    #[tokio::test]
    async fn request_get_balance() {
        let environment = RevmEnvironment::new("test".to_string());
        let revm_middleware = RevmMiddleware::new(environment);
        let from = Address::zero();
        let block = Some(BlockId::Number(1.into()));
        let balance = revm_middleware.get_balance(from, block).await.unwrap();
    }
}
