#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::core::types::U64;
use ethers::providers::{JsonRpcClient, ProviderError};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, U256},
    EVM,
};
use RustQuant::statistics::distributions::i;

use crate::{
    agent::{Agent, IsAttached, NotAttached},
    math::stochastic_process::{sample_poisson, SeededPoisson},
    middleware::RevmMiddleware,
    utils::{convert_uint_to_u64, revm_logs_to_ethers_logs},
};
use serde::{de::DeserializeOwned, Serialize};

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
    pub(crate) tx_sender: TxEnvSender,
    pub tx_receiver: TxEnvReceiver,
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
    /// Clients (Agents) in the environment
    pub agents: Vec<Agent<IsAttached<RevmMiddleware>>>,
    /// expected events per block
    pub seeded_poisson: SeededPoisson,
}

// TODO: If the provider holds the connection then this can work better.
#[derive(Clone)]
pub struct RevmProvider {
    pub(crate) tx_sender: TxEnvSender,
    pub(crate) result_sender: crossbeam_channel::Sender<RevmResult>,
    pub(crate) result_receiver: crossbeam_channel::Receiver<RevmResult>,
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
        _params: T,
    ) -> Result<R, ProviderError> {
        match method {
            "eth_getFilterChanges" => {
                let logs = self.event_receiver.recv().unwrap();
                println!("logs: {:?}", logs);
                let logs_str = serde_json::to_string(&logs).unwrap();
                let logs_deserializeowned: R = serde_json::from_str(&logs_str)?;
                return Ok(logs_deserializeowned);
                // return Ok(serde::to_value(self.event_receiver.recv().ok()).unwrap())
            }
            _ => {
                unimplemented!("We don't cover this case yet.")
            }
        }
    }
}

impl Environment {
    /// Creates a new [`Environment`] with the given label.
    pub(crate) fn new<S: Into<String>>(label: S, block_rate: f64, seed: u64) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);

        let seeded_poisson = SeededPoisson::new(block_rate, seed);
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.env.block.gas_limit = U256::MAX;
        let (tx_sender, tx_receiver) = unbounded::<(ToTransact, TxEnv, Sender<RevmResult>)>();
        Self {
            label: label.into(),
            state: State::Stopped,
            evm,
            tx_sender,
            tx_receiver,
            event_broadcaster: Arc::new(Mutex::new(EventBroadcaster::new())),
            agents: vec![],
            seeded_poisson,
        }
    }

    /// Creates a new [`Agent<RevmMiddleware`] with the given label.
    // TODO: We need to make this the way to add agents to the environment.
    // in `agent.rs` we have `new_simulation_agent` which should probably just be called from this function instead.
    // OR agents can be created (without a connection?) and then added to the environment where they will gain a connection?
    // Waylon: I like them being created without a connection and then added to the environment where they will gain a connection.
    pub fn add_agent(&mut self, agent: Agent<NotAttached>) {
        agent.attach_to_environment(self);
    }

    // TODO: Run should now run the agents as well as the evm.
    pub(crate) fn run(&mut self) {
        let tx_receiver = self.tx_receiver.clone();
        let mut evm = self.evm.clone();
        let event_broadcaster = self.event_broadcaster.clone();

        let mut seeded_poisson = self.seeded_poisson.clone();

        let mut counter: usize = 0;
        self.state = State::Running;

        thread::spawn(move || {
            let mut expected_events_per_block = seeded_poisson.sample();

            while let Ok((to_transact, tx, sender)) = tx_receiver.recv() {
                // Execute the transaction, echo the logs to all agents, and report the execution result to the agent who made the transaction.
                if counter == expected_events_per_block {
                    counter = 0;
                    println!("EVM expected number of transactions reached. Moving to next block.");
                    println!("old block number: {:?}", evm.env.block.number);
                    evm.env.block.number += U256::from(1);
                    println!("new block number: {:?}", evm.env.block.number);
                    expected_events_per_block = seeded_poisson.sample();
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
                    let revm_result = RevmResult {
                        result: execution_result,
                        block_number: convert_uint_to_u64(evm.env.block.number).unwrap(),
                    };
                    sender.send(revm_result).unwrap();
                    counter += 1;
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
    use std::time::Duration;

    use anyhow::{Ok, Result};
    use ethers::types::Address;

    use crate::bindings::arbiter_token::ArbiterToken;

    use super::*;

    pub(crate) const TEST_ENV_LABEL: &str = "test";

    #[test]
    fn new() {
        let env = Environment::new(TEST_ENV_LABEL.to_string(), 1.0, 1);
        assert_eq!(env.label, TEST_ENV_LABEL);
        assert_eq!(env.state, State::Stopped);
    }

    #[test]
    fn run() {
        let mut environment = Environment::new(TEST_ENV_LABEL.to_string(), 1.0, 1);
        environment.run();
        assert_eq!(environment.state, State::Running);
    }
}
