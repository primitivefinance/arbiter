#![warn(missing_docs)]
#![warn(unsafe_code)]

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::{
    middleware::providers::{JsonRpcClient, RpcError},
    providers::{MockProvider, Provider},
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockId, Bytes, Filter, NameOrAddress,
        H256, U64,
    },
};
use ethers_middleware::providers::{
    FilterKind, FilterWatcher, Middleware, MiddlewareError, PendingTransaction, ProviderError,
};
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

// impl std::fmt::Display for RevmEnvironmentError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Implement how you want to display the error
//         // Example: write!(f, "RevmEnvironmentError: {}", self)
//         // You can also use the `thiserror` derive to customize the formatting
//         write!(f, "{}", self)
//     }
// }

impl MiddlewareError for RevmEnvironmentError {
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
        unreachable!("There is no need to make a raw serialized request here")
    }
}

// TODO: At the moment we should use the `fill_transaction()` method for doing transactions.
// However, it is not totally implemented and will require higher middleware stack for agents.
// Also, we will need to figure out what to do with `ExecutionResult`s.
#[async_trait::async_trait]
impl Middleware for RevmEnvironment {
    type Error = RevmEnvironmentError;
    type Provider = Self;
    type Inner = Self;

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn provider(&self) -> &Provider<Self::Provider> {
        unreachable!("There is no inner provider here")
    }

    fn default_sender(&self) -> Option<Address> {
        // TODO: We should implement a default sender here so that deploys, for example, can be done more easily.
        Some(Address::from_low_u64_be(1))
    }

    async fn get_block_number(&self) -> Result<U64, Self::Error> {
        todo!()
        // self.evm.env.block.number().map_err(|_| Self::Error)
    }

    async fn get_gas_price(&self) -> Result<ethers::types::U256, Self::Error> {
        todo!()
        // self.evm.env.gas_price().map_err(|_| Self::Error)
    }

    async fn get_balance<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<ethers::types::U256, Self::Error> {
        todo!()
        // self.evm.db().accounts.get(&from.into()).map(|account| account.balance)
    }

    async fn fill_transaction(
        &self,
        tx: &mut TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<(), Self::Error> {
        let tx_sender = &self.transaction_channel.0;
        let tx_env = TxEnv::default();
        tx_sender.send((tx_env, crossbeam_channel::unbounded().0)).unwrap();
        Ok(())
    }

    async fn get_logs(&self, filter: &Filter) -> Result<Vec<ethers::types::Log>, Self::Error> {
        todo!()
    }

    async fn watch<'a>(
        &'a self,
        filter: &Filter,
    ) -> Result<FilterWatcher<'a, Self::Provider, ethers::types::Log>, Self::Error> {
        todo!()
    }

    async fn watch_blocks(&self) -> Result<FilterWatcher<'_, Self::Provider, H256>, Self::Error> {
        todo!()
    }

    async fn new_filter(&self, filter: FilterKind<'_>) -> Result<ethers::types::U256, Self::Error> {
        todo!()
    }

    async fn uninstall_filter<T: Into<ethers::types::U256> + Send + Sync>(
        &self,
        id: T,
    ) -> Result<bool, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::writer::Writer;
    use anyhow::Result;

    const TEST_LABEL: &str = "test";

    #[test]
    fn new() {
        let env = RevmEnvironment::new(TEST_LABEL.to_string());
        assert_eq!(env.label, TEST_LABEL);
        assert_eq!(env.state, State::Stopped);
    }

    #[test]
    fn run() {
        let mut environment = RevmEnvironment::new(TEST_LABEL.to_string());
        environment.run();
        assert_eq!(environment.state, State::Running);
    }

    #[tokio::test]
    async fn deploy_transaction() -> Result<()> {
        let environment = Arc::new(RevmEnvironment::new(TEST_LABEL.to_string()));
        let writer = Writer::deploy(environment, ())?.send().await;
        println!("{:#?}", writer);
        Ok(())
    }
}
