#![warn(missing_docs)]
#![warn(unsafe_code)]

// use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::core::types::U64;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, TxEnv, U256},
    EVM,
};
use std::fmt::Debug;

use crate::{
    agent::{Agent, IsAttached, NotAttached},
    math::stochastic_process::SeededPoisson,
    middleware::RevmMiddleware,
    utils::convert_uint_to_u64,
};
use tokio::sync::broadcast;

/// Result struct for the [`Environment`]. that wraps the [`ExecutionResult`] and the block number.
#[derive(Debug, Clone)]
pub(crate) struct RevmResult {
    pub(crate) result: ExecutionResult,
    pub(crate) block_number: U64,
}

pub(crate) type ToTransact = bool;
pub(crate) type ResultSender = crossbeam_channel::Sender<RevmResult>;
pub(crate) type ResultReceiver = crossbeam_channel::Receiver<RevmResult>;
pub(crate) type TxSender = crossbeam_channel::Sender<(ToTransact, TxEnv, ResultSender)>;
pub(crate) type TxReceiver = crossbeam_channel::Receiver<(ToTransact, TxEnv, ResultSender)>;
pub(crate) type EventBroadcaster = broadcast::Sender<Vec<ethers::types::Log>>;

/// State enum for the [`Environment`].
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum State {
    /// The [`Environment`] is currently running.
    /// [`Agent`]s cannot be added if the environment is [`State::Running`].
    Running,
    /// The [`Environment`] is currently stopped.
    /// [`Agent`]s can only be added if the environment is [`State::Initialization`].
    Initialization,
}

pub(crate) struct Socket {
    pub(crate) tx_sender: TxSender,
    tx_receiver: TxReceiver,
    pub(crate) event_sender: EventBroadcaster,
}

/// The environment struct.
pub struct Environment {
    pub label: String,
    pub(crate) state: State,
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    pub(crate) socket: Socket,
    pub agents: Vec<Agent<IsAttached<RevmMiddleware>>>,
    pub seeded_poisson: SeededPoisson,
}

// TODO: This could be improved.
impl Debug for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Socket").finish()
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

        let (tx_sender, tx_receiver) = crossbeam_channel::bounded(16);
        let (event_sender, _) = tokio::sync::broadcast::channel(16);
        let socket = Socket {
            tx_sender,
            tx_receiver,
            event_sender,
        };

        Self {
            label: label.into(),
            state: State::Initialization,
            evm,
            socket,
            agents: vec![],
            seeded_poisson,
        }
    }

    /// Creates a new [`Agent<RevmMiddleware`] with the given label.
    pub fn add_agent(&mut self, agent: Agent<NotAttached>) {
        agent.attach_to_environment(self);
    }

    // TODO: Run should now run the agents as well as the evm.
    pub(crate) fn run(&mut self) {
        let mut evm = self.evm.clone();
        let tx_receiver = self.socket.tx_receiver.clone();
        let event_broadcaster = self.socket.event_sender.clone();

        let mut seeded_poisson = self.seeded_poisson.clone();

        let mut counter: usize = 0;
        self.state = State::Running;

        std::thread::spawn(move || {
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

                    event_broadcaster.send(crate::utils::revm_logs_to_ethers_logs(
                        execution_result.logs(),
                    ));
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

#[cfg(test)]
pub(crate) mod tests {

    use super::*;

    pub(crate) const TEST_ENV_LABEL: &str = "test";

    #[test]
    fn new() {
        let env = Environment::new(TEST_ENV_LABEL.to_string(), 1.0, 1);
        assert_eq!(env.label, TEST_ENV_LABEL);
        assert_eq!(env.state, State::Initialization);
    }

    #[test]
    fn run() {
        let mut environment = Environment::new(TEST_ENV_LABEL.to_string(), 1.0, 1);
        environment.run();
        assert_eq!(environment.state, State::Running);
    }
}
