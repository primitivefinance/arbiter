#![warn(missing_docs)]
#![warn(unsafe_code)]

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::{core::types::U64, types::Log};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, TxEnv, U256},
    EVM,
};
use std::{
    fmt::Debug,
    sync::{Arc, Condvar, Mutex},
    thread,
};

use crate::{
    agent::{Agent, IsAttached, NotAttached},
    math::stochastic_process::SeededPoisson,
    middleware::RevmMiddleware,
    utils::convert_uint_to_u64,
};
// use tokio::sync::broadcast;

/// Result struct for the [`Environment`]. that wraps the [`ExecutionResult`] and the block number.
#[derive(Debug, Clone)]
pub(crate) struct RevmResult {
    pub(crate) result: ExecutionResult,
    pub(crate) block_number: U64,
}

pub(crate) type ToTransact = bool;
pub(crate) type ResultSender = Sender<RevmResult>;
pub(crate) type ResultReceiver = Receiver<RevmResult>;
pub(crate) type TxSender = Sender<(ToTransact, TxEnv, ResultSender)>;
pub(crate) type TxReceiver = Receiver<(ToTransact, TxEnv, ResultSender)>;

#[atomic_enum::atomic_enum]
#[derive(Eq, PartialEq)]
pub enum State {
    /// The [`Environment`] is currently stopped.
    /// [`Agent`]s can only be added if the environment is [`State::Initialization`].
    Initialization,
    /// The [`Environment`] is currently running.
    /// [`Agent`]s cannot be added if the environment is [`State::Running`].
    Running,
    Paused,
    Stopped,
}

pub(crate) struct Socket {
    pub(crate) tx_sender: TxSender,
    pub(crate) tx_receiver: TxReceiver,
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
}

pub struct Environment {
    pub label: String,
    pub(crate) state: Arc<AtomicState>,
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    pub(crate) socket: Socket,
    pub agents: Vec<Agent<IsAttached<RevmMiddleware>>>,
    pub seeded_poisson: SeededPoisson,
    pub(crate) pausevar: Arc<(Mutex<()>, Condvar)>,
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

        let (tx_sender, tx_receiver) = unbounded();

        let socket = Socket {
            tx_sender,
            tx_receiver,
            event_broadcaster: Arc::new(Mutex::new(EventBroadcaster::new())),
        };

        Self {
            label: label.into(),
            state: Arc::new(AtomicState::new(State::Initialization)),
            evm,
            socket,
            agents: vec![],
            seeded_poisson,
            pausevar: Arc::new((Mutex::new(()), Condvar::new())),
        }
    }

    /// Creates a new [`Agent<RevmMiddleware`] with the given label.
    pub fn add_agent(&mut self, agent: Agent<NotAttached>) {
        agent.attach_to_environment(self);
    }

    // TODO: Run should now run the agents as well as the evm.
    pub(crate) fn run(&mut self) -> std::thread::JoinHandle<()> {
        let mut evm = self.evm.clone();
        let tx_receiver = self.socket.tx_receiver.clone();
        let event_broadcaster = self.socket.event_broadcaster.clone();

        let mut seeded_poisson = self.seeded_poisson.clone();

        let mut counter: usize = 0;
        self.state
            .store(State::Running, std::sync::atomic::Ordering::Relaxed);
        let state = Arc::clone(&self.state);
        let pausevar = Arc::clone(&self.pausevar);

        thread::spawn(move || {
            let mut expected_events_per_block = seeded_poisson.sample();
            loop {
                // println!("Looping.");
                match state.load(std::sync::atomic::Ordering::Relaxed) {
                    State::Stopped => {
                        println!("Entered stopped state.");
                        break
                    },
                    State::Paused => {
                        println!("Entered paused state.");
                        let (lock, cvar) = &*pausevar;
                        let mut guard = lock.lock().unwrap();
                        while state.load(std::sync::atomic::Ordering::Relaxed) == State::Paused {
                            guard = cvar.wait(guard).unwrap();
                        }
                        println!("Exiting paused state.");
                    }
                    State::Running => {
                        // println!("Running evm.");
                        if let Ok((to_transact, tx, sender)) = tx_receiver.recv() {
                            if counter == expected_events_per_block {
                                counter = 0;
                                evm.env.block.number += U256::from(1);
                                expected_events_per_block = seeded_poisson.sample();
                            }

                            evm.env.tx = tx;
                            if to_transact {
                                let execution_result = match evm.transact_commit() {
                                    Ok(val) => val,
                                    // URGENT: change this to a custom error
                                    Err(_) => panic!("failed"),
                                };
                                let event_broadcaster = event_broadcaster.lock().unwrap();
                                event_broadcaster.broadcast(
                                    crate::utils::revm_logs_to_ethers_logs(execution_result.logs()),
                                );
                                let revm_result = RevmResult {
                                    result: execution_result,
                                    block_number: convert_uint_to_u64(evm.env.block.number)
                                        .unwrap(),
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
                                    block_number: convert_uint_to_u64(evm.env.block.number)
                                        .unwrap(),
                                };
                                sender.send(result_and_block).unwrap();
                            }
                        }
                    }
                    State::Initialization => {
                        panic!("Environment is in an invalid state: Initialization. This should not be possible.");
                    }
                }
            }
        })
    }
}

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

    pub(crate) const TEST_ENV_LABEL: &str = "test";

    #[test]
    fn new() {
        let environment = Environment::new(TEST_ENV_LABEL.to_string(), 1.0, 1);
        assert_eq!(environment.label, TEST_ENV_LABEL);
        let state = environment.state.load(std::sync::atomic::Ordering::Relaxed);
        assert_eq!(state, State::Initialization);
    }

    #[test]
    fn run() {
        let mut environment = Environment::new(TEST_ENV_LABEL.to_string(), 1.0, 1);
        environment.run();
        let state = environment.state.load(std::sync::atomic::Ordering::Relaxed);
        assert_eq!(state, State::Running);
    }
}
