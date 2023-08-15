#![warn(missing_docs, unsafe_code)]

// TODO: Add logging especially inside of the run function. This will be necessary for pausing and debugging.
// TODO: Add custom errors.
// TODO: Check the publicness of all structs and functions.

use std::{
    convert::Infallible,
    fmt::Debug,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::{core::types::U64, types::Log};
use log::error;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{EVMError, ExecutionResult, TxEnv, U256},
    EVM,
};
use thiserror::Error;

use crate::{math::*, middleware::RevmMiddleware};

pub(crate) type ToTransact = bool;
pub(crate) type ResultSender = Sender<RevmResult>;
pub(crate) type ResultReceiver = Receiver<RevmResult>;
pub(crate) type TxSender = Sender<(ToTransact, TxEnv, ResultSender)>;
pub(crate) type TxReceiver = Receiver<(ToTransact, TxEnv, ResultSender)>;
pub(crate) type EventSender = Sender<Vec<Log>>;

pub struct Environment {
    pub label: String,
    pub(crate) state: Arc<AtomicState>,
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    pub(crate) socket: Socket,
    pub seeded_poisson: Option<SeededPoisson>,
    pub(crate) handle: Option<JoinHandle<Result<(), EnvironmentError>>>,
    pub(crate) pausevar: Arc<(Mutex<()>, Condvar)>,
}

#[derive(Error, Debug)]
pub enum EnvironmentError {
    #[error("execution error! the source error is: {cause:?}")]
    ExecutionError { cause: EVMError<Infallible> },

    #[error("error pausing! the source error is: {cause:?}")]
    PauseError { cause: String },

    #[error("error communicating! the source error is: {cause:?}")]
    CommunicationError { cause: String },

    #[error("conversion error! the source error is: {cause:?}")]
    ConversionError { cause: String },
}

impl Environment {
    pub(crate) fn new<S: Into<String>>(label: S, block_rate: Option<f64>, seed: u64) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);
        let mut seeded_poisson;
        if let Some(block_rate) = block_rate {
            seeded_poisson = Some(SeededPoisson::new(block_rate, seed));
        } else {
            seeded_poisson = None;
        }
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
            seeded_poisson,
            handle: None,
            pausevar: Arc::new((Mutex::new(()), Condvar::new())),
        }
    }

    pub(crate) fn run(&mut self) {
        let mut evm = self.evm.clone();
        let tx_receiver = self.socket.tx_receiver.clone();
        let event_broadcaster = self.socket.event_broadcaster.clone();

        let mut seeded_poisson_option = self.seeded_poisson.clone();

        let mut counter: u64 = 0;
        self.state
            .store(State::Running, std::sync::atomic::Ordering::Relaxed);
        let state = Arc::clone(&self.state);
        let pausevar = Arc::clone(&self.pausevar);
        let label = self.label.clone();

        let handle = thread::spawn(move || {
            let mut expected_events_per_block;
            if let Some(ref mut seeded_poisson) = seeded_poisson_option {
                expected_events_per_block = seeded_poisson.sample();
            } else {
                expected_events_per_block = u64::MAX;
            }
            loop {
                match state.load(std::sync::atomic::Ordering::Relaxed) {
                    State::Stopped => break,
                    State::Paused => {
                        let (lock, cvar) = &*pausevar;
                        let mut guard = lock.lock().map_err(|e| EnvironmentError::PauseError {
                            cause: format!("{:?}", e),
                        })?;
                        while state.load(std::sync::atomic::Ordering::Relaxed) == State::Paused {
                            guard = cvar.wait(guard).map_err(|e| EnvironmentError::PauseError {
                                cause: format!("{:?}", e),
                            })?;
                        }
                    }
                    State::Running => {
                        if let Ok((to_transact, tx, sender)) = tx_receiver.recv() {
                            if counter == expected_events_per_block {
                                counter = 0;
                                evm.env.block.number += U256::from(1);
                                if let Some(ref mut seeded_poisson) = seeded_poisson_option {
                                    expected_events_per_block = seeded_poisson.sample();
                                }
                            }

                            evm.env.tx = tx;
                            if to_transact {
                                let execution_result = match evm.transact_commit() {
                                    // Check for an error in execution ([`EVMError<Infallible>`]),
                                    // but pass to the middleware to determine if the result is
                                    // [`ExecutionResult::Success`], [`ExecutionResult::Revert`], or [`ExecutionResult::Halt`].
                                    Ok(val) => val,
                                    Err(e) => {
                                        state.store(
                                            State::Paused,
                                            std::sync::atomic::Ordering::Relaxed,
                                        );
                                        error!("Pausing the environment labeled {} due to an execution error: {:#?}", label, e);
                                        return Err(EnvironmentError::ExecutionError { cause: e });
                                    }
                                };
                                let event_broadcaster = event_broadcaster.lock().map_err(|e| {
                                    EnvironmentError::CommunicationError {
                                        cause: format!("{:?}", e),
                                    }
                                })?;
                                event_broadcaster.broadcast(
                                    crate::middleware::revm_logs_to_ethers_logs(
                                        execution_result.logs(),
                                    ),
                                )?;
                                let revm_result = RevmResult {
                                    result: execution_result,
                                    block_number: convert_uint_to_u64(evm.env.block.number)
                                        .map_err(|e| EnvironmentError::ConversionError {
                                            cause: format!("{:?}", e),
                                        })?,
                                };
                                sender.send(revm_result).map_err(|e| {
                                    EnvironmentError::CommunicationError {
                                        cause: format!("{:?}", e),
                                    }
                                })?;
                                counter += 1;
                            } else {
                                let result = match evm.transact() {
                                    // Check for an error in execution ([`EVMError<Infallible>`]),
                                    // but pass to the middleware to determine if the result is
                                    // [`ExecutionResult::Success`], [`ExecutionResult::Revert`], or [`ExecutionResult::Halt`].
                                    Ok(result_and_state) => result_and_state.result,
                                    Err(e) => {
                                        state.store(
                                            State::Paused,
                                            std::sync::atomic::Ordering::Relaxed,
                                        );
                                        error!("Pausing the environment labeled {} due to an execution error: {:#?}", label, e);
                                        return Err(EnvironmentError::ExecutionError { cause: e });
                                    }
                                };
                                let result_and_block = RevmResult {
                                    result,
                                    block_number: convert_uint_to_u64(evm.env.block.number)
                                        .map_err(|e| EnvironmentError::ConversionError {
                                            cause: format!("{:?}", e),
                                        })?,
                                };
                                sender.send(result_and_block).map_err(|e| {
                                    EnvironmentError::CommunicationError {
                                        cause: format!("{:?}", e),
                                    }
                                })?;
                            }
                        }
                    }
                    State::Initialization => {
                        panic!("Environment is in an invalid state: Initialization. This should not be possible.");
                    }
                }
            }
            Ok(())
        });
        self.handle = Some(handle);
    }
}

#[atomic_enum::atomic_enum]
#[derive(Eq, PartialEq)]
pub enum State {
    Initialization,
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Clone)]
pub(crate) struct Socket {
    pub(crate) tx_sender: TxSender,
    pub(crate) tx_receiver: TxReceiver,
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
}

#[derive(Debug, Clone)]
pub(crate) struct RevmResult {
    pub(crate) result: ExecutionResult,
    pub(crate) block_number: U64,
}

#[derive(Clone, Debug)]
pub struct EventBroadcaster(Vec<EventSender>);

impl EventBroadcaster {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }

    pub(crate) fn add_sender(&mut self, sender: EventSender) {
        self.0.push(sender);
    }

    pub(crate) fn broadcast(&self, logs: Vec<Log>) -> Result<(), EnvironmentError> {
        for sender in &self.0 {
            sender
                .send(logs.clone())
                .map_err(|e| EnvironmentError::CommunicationError {
                    cause: format!("{:?}", e),
                })?;
        }
        Ok(())
    }
}

/// Convert a U256 to a U64, discarding the higher bits if the number is larger than 2^64
/// # Arguments
/// * `input` - The U256 to convert.
/// # Returns
/// * `Ok(U64)` - The converted U64.
/// Used for block number which is a U64.
#[inline]
pub fn convert_uint_to_u64(input: U256) -> Result<U64, &'static str> {
    let as_str = input.to_string();
    match as_str.parse::<u64>() {
        Ok(val) => Ok(val.into()),
        Err(_) => Err("U256 value is too large to fit into u64"),
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;

    pub(crate) const TEST_ENV_LABEL: &str = "test";

    #[test]
    fn new() {
        let environment = Environment::new(TEST_ENV_LABEL.to_string(), Some(1.0), 1);
        assert_eq!(environment.label, TEST_ENV_LABEL);
        let state = environment.state.load(std::sync::atomic::Ordering::Relaxed);
        assert_eq!(state, State::Initialization);
    }

    #[test]
    fn run() {
        let mut environment = Environment::new(TEST_ENV_LABEL.to_string(), Some(1.0), 1);
        environment.run();
        let state = environment.state.load(std::sync::atomic::Ordering::Relaxed);
        assert_eq!(state, State::Running);
    }

    #[test]
    fn test_conversion() {
        // Test with a value that fits in u64.
        let input = U256::from(10000);
        assert_eq!(convert_uint_to_u64(input).unwrap(), U64::from(10000));

        // Test with a value that is exactly at the limit of u64.
        let input = U256::from(u64::MAX);
        assert_eq!(convert_uint_to_u64(input).unwrap(), U64::from(u64::MAX));

        // Test with a value that exceeds the limit of u64.
        let input = U256::from(u64::MAX) + U256::from(1);
        assert!(convert_uint_to_u64(input).is_err());
    }
}
