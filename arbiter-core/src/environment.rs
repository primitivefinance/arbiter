//! The `environment` module provides abstractions and functionality for
//! handling the Ethereum execution environment. This includes managing its
//! state, interfacing with the EVM, and broadcasting events to subscribers.
//!
//! The key integration for the environment is the Rust EVM [`revm`](https://github.com/bluealloy/revm).
//! This is an implementation of the EVM in Rust that we utilize for processing
//! raw smart contract bytecode.
//!
//! Core structures:
//! - `Environment`: Represents the Ethereum execution environment, allowing for
//!   its management (e.g., starting, stopping) and interfacing with agents.
//! - `State`: Enum indicating the current state of the environment.
//! - `Socket`: Provides channels for communication between the EVM and the
//!   outside world.
//! - `RevmResult`: Wraps the result of a transaction along with the block
//!   number.
//! - `EventBroadcaster`: Responsible for broadcasting Ethereum logs to
//!   subscribers.

#![warn(missing_docs, unsafe_code)]

use std::{
    convert::Infallible,
    fmt::Debug,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::core::types::U64;
use log::error;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{EVMError, ExecutionResult, Log, TxEnv, U256},
    EVM,
};
use thiserror::Error;

use crate::math::SeededPoisson;
#[cfg_attr(doc, doc(hidden))]
#[cfg_attr(doc, allow(unused_imports))]
#[cfg(doc)]
use crate::{manager::Manager, middleware::RevmMiddleware};

/// Alias to represent that a transaction sent to the
/// [`EVM`](https://docs.rs/revm/3.3.0/revm/struct.EVM.html) updates the
/// worldstate (`true`) or is read only (`false`)
pub(crate) type ToTransact = bool;

/// Alias for the sender of the channel for transmitting [`RevmResult`] emitted
/// from transactions.
pub(crate) type ResultSender = Sender<RevmResult>;

/// Alias for the receiver of the channel for transmitting [`RevmResult`]
/// emitted from transactions.
pub(crate) type ResultReceiver = Receiver<RevmResult>;

/// Alias for the sender of the channel for transmitting transactions.
pub(crate) type TxSender = Sender<(ToTransact, TxEnv, ResultSender)>;

/// Alias for the receiver of the channel for transmitting transactions.
pub(crate) type TxReceiver = Receiver<(ToTransact, TxEnv, ResultSender)>;

/// Alias for the sender used in the [`EventBroadcaster`] that transmits
/// contract events via [`Log`].
pub(crate) type EventSender = Sender<Vec<Log>>;

/// Represents a [`Manager`]-controllable version of the Ethereum execution
/// environment.
///
/// ## Communication
/// The dominant feature is the
/// [`EVM`](https://github.com/bluealloy/revm/blob/main/crates/revm/src/evm.rs)
///  and its connections to the "outside world".
/// The Ethereum Virtual Machine
/// ([`EVM`](https://github.com/bluealloy/revm/blob/main/crates/revm/src/evm.rs))
/// which is a stack machine that processes raw smart contract bytecode and
/// updates a local database of the worldstate of an Ethereum simulation.
/// Note, the worldstate of the simulation Ethereum environment should not be
/// confused with the [`State`] of the environment here! The [`Environment`]
/// will route transactions sent over channels to the stack machine
/// [`EVM`](https://github.com/bluealloy/revm/blob/main/crates/revm/src/evm.rs)
/// to process smart contract interactions.
///
/// Allows for the initialization, starting, stopping, and pausing of the EVM
/// execution. It provides channels for sending transactions to the EVM and for
/// receiving results or broadcasting events to any subscribers via the
/// `Socket` field exposed only as `pub(crate)`.
///
/// ## Status
/// The environment also maintains its current
/// state, which can be one of the following:
/// - [`State::Initialization`],
/// - [`State::Running`],
/// - [`State::Paused`],
/// - [`State::Stopped`].
///
/// A state of [`State::Paused`] is recoverable and a state of
/// [`State::Stopped`] is not recoverable. [`State::Initialization`] is adopted
/// only prior to the [`Environment`] being ran and marks that this
/// [`Environment`] may still change in configuration.
///
/// ## Controlling Block Rate
/// The blocks for the [`Environment`] are chosen using a Poisson distribution
/// via the [`SeededPoisson`] field. The idea is that we can choose a rate
/// paramater, typically denoted by the Greek letter lambda, and set this to be
/// the expected number of transactions per block while allowing blocks to be
/// built with random size. This is useful in stepping forward the
/// [`EVM`](https://github.com/bluealloy/revm/blob/main/crates/revm/src/evm.rs)
/// and being able to move time forward for contracts that depend explicitly on
/// time.
pub struct Environment {
    /// A label for the [`Environment`].
    /// Used to allow the [`Manager`] to locate the [`Environment`] in order to
    /// control it. Also used to be able to organize, track progress, and
    /// post-process results.
    pub label: String,

    /// A seeded Poisson distribution that is sampled from in order to determine
    /// the average block size. [`SeededPoisson`] is created with a seed in
    /// order to have repeatable simulations.
    pub seeded_poisson: SeededPoisson,

    // Private fields
    /// The [`State`] of the [`Environment`] which is shared across threads,
    /// hence the [`Arc`] wrapper. [`State`] can be changed manually using
    /// the [`Manager`] or upon running into errors.
    pub(crate) state: Arc<AtomicState>,

    /// The [`EVM`] that is used as an execution environment and database for
    /// calls and transactions.
    evm: EVM<CacheDB<EmptyDB>>,

    /// This gives a means of letting the "outside world" connect to the
    /// [`Environment`] so that users (or agents) may send and receive data from
    /// the [`EVM`].
    pub(crate) socket: Socket,

    /// [`Condvar`] to allow for the [`Manager`] or errors to set the state of
    /// the [`Environment`] to [`State::Paused`] for debugging or other reasons.
    pub(crate) pausevar: Arc<(Mutex<()>, Condvar)>,

    /// [`JoinHandle`] for the thread in which the [`EVM`] is running.
    /// Used for assuring that the environment is stopped properly or for
    /// performing any blocking action the end user needs.
    pub(crate) handle: Option<JoinHandle<Result<(), EnvironmentError>>>,
}

/// Allow the end user to be able to access a debug printout for the
/// [`Environment`]. Note that the [`EVM`] does not implement debug display,
/// hence the implementation by hand here.
impl Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Environment")
            .field("label", &self.label)
            .field("seeded_poisson", &self.seeded_poisson)
            .field("state", &self.state)
            .field("socket", &self.socket)
            .field("pausevar", &self.pausevar)
            .field("handle", &self.handle)
            .finish()
    }
}

/// Errors that can occur when managing or interfacing with the Ethereum
/// environment.
///
/// ## What are we trying to catcH?
/// The errors here are at a fairly low level and should be quite rare (if
/// possible). Errors that come from smart contracts (e.g., reverts or halts)
/// will not be caught here and will instead carried out into the
/// [`RevmMiddleware`]. Please bring up if you catch errors here by sending a
/// message in the [Telegram group](https://t.me/arbiter_rs) or on
/// [GitHub](https://github.com/primitivefinance/arbiter/).
#[derive(Error, Debug, Clone)]
pub enum EnvironmentError {
    /// [`EnvironmentError::Execution`] is thrown when the [`EVM`] itself
    /// throws an error in execution. To be clear, this is not a contract
    /// revert or halt, this is likely an error in `revm`. Please report
    /// this type of error.
    #[error("execution error! the source error is: {0:?}")]
    Execution(EVMError<Infallible>),

    /// [`EnvironmentError::Pause`] is thrown when the [`Environment`]
    /// fails to pause. This should likely never occur, but if it does,
    /// please report this error!
    #[error("error pausing! the source error is: {0}")]
    Pause(String),

    /// [`EnvironmentError::Communication`] is thrown when a channel for
    /// receiving or broadcasting fails in some way. This error could happen
    /// due to a channel being closed accidentally. If this is thrown, a
    /// restart of the simulation and an investigation into what caused a
    /// dropped channel is necessary.
    #[error("error communicating! the source error is: {0}")]
    Communication(String),

    /// [`EnvironmentError::Conversion`] is thrown when a type fails to
    /// convert into another (typically a type used in `revm` versus a type used
    /// in [`ethers-rs`](https://github.com/gakonst/ethers-rs)).
    /// This error should be rare (if not impossible).
    /// Furthermore, after a switch to [`alloy`](https://github.com/alloy-rs)
    /// this will be (hopefully) unnecessary!
    #[error("conversion error! the source error is: {0}")]
    Conversion(String),
}

impl Environment {
    /// Privately accessible constructor function for creating an
    /// [`Environment`]. This function should be accessed by the
    /// [`Manager`].
    pub(crate) fn new<S: Into<String>>(label: S, block_rate: f64, seed: u64) -> Self {
        // Initialize the EVM used
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);

        // Chooose extra large code size and gas limit
        evm.env.cfg.limit_contract_code_size = Some(0x100000);
        evm.env.block.gas_limit = U256::MAX;

        let seeded_poisson = SeededPoisson::new(block_rate, seed);

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

    /// Privately accessible function to take an [`Environment`] that is in
    /// [`State::Initialization`] and start it running. The [`EVM`] will be
    /// offloaded onto a separate thread for processing.
    /// Calls, transactions, and events will enter/exit through the `Socket`.
    /// Upon calling this function, the [`Environment`] will be placed in
    /// [`State::Running`]. Errors here may trigger the [`Environment`] to
    /// be placed in [`State::Paused`].
    pub(crate) fn run(&mut self) {
        // Pull clones of the relevant data prepare to send into a new thread
        let label = self.label.clone();
        let mut evm = self.evm.clone();
        let tx_receiver = self.socket.tx_receiver.clone();
        let event_broadcaster = self.socket.event_broadcaster.clone();
        let mut seeded_poisson = self.seeded_poisson.clone();

        // Set up the state and tx counter
        self.state
            .store(State::Running, std::sync::atomic::Ordering::SeqCst);
        let state = Arc::clone(&self.state);
        let pausevar = Arc::clone(&self.pausevar);
        let mut counter: usize = 0;

        // Move the EVM and its socket to a new thread and retrieve this handle
        let handle = thread::spawn(move || {
            // Get the first amount of transactions per block from the distribution
            let mut transactions_per_block = seeded_poisson.sample();

            // Loop over the reception of calls/transactions sent through the socket
            loop {
                // The outermost check is to find what the `Environment`'s state is in
                match state.load(std::sync::atomic::Ordering::SeqCst) {
                    // Leave the loop upon seeing `State::Stopped`
                    State::Stopped => break,

                    // Await for the condvar alert to change the state
                    State::Paused => {
                        let (lock, cvar) = &*pausevar;
                        let mut guard = lock
                            .lock()
                            .map_err(|e| EnvironmentError::Pause(format!("{:?}", e)))?;

                        // this logic here ensures we catch any edge case last transactions and send
                        // the appropriate error so that we dont hang in
                        // limbo forever
                        while let Ok((_, _, sender)) = tx_receiver.try_recv() {
                            let error_outcome = TransactionOutcome::Error(EnvironmentError::Pause(
                                "Environment is paused".into(),
                            ));
                            let revm_result = RevmResult {
                                outcome: error_outcome,
                                block_number: convert_uint_to_u64(evm.env.block.number).map_err(
                                    |e| EnvironmentError::Conversion(format!("{:?}", e)),
                                )?,
                            };
                            sender
                                .send(revm_result)
                                .map_err(|e| EnvironmentError::Communication(format!("{:?}", e)))?;
                        }

                        while state.load(std::sync::atomic::Ordering::SeqCst) == State::Paused {
                            guard = cvar
                                .wait(guard)
                                .map_err(|e| EnvironmentError::Pause(format!("{:?}", e)))?;
                        }
                    }

                    // Receive new transactions
                    State::Running => {
                        if let Ok((to_transact, tx, sender)) = tx_receiver.recv() {
                            // Check whether we need to increment the block number given the amount
                            // of transactions that have occured on the current block and increment
                            // if need be and draw a new sample from the `SeededPoisson`
                            // distribution
                            if counter == transactions_per_block {
                                counter = 0;
                                evm.env.block.number += U256::from(1);
                                transactions_per_block = seeded_poisson.sample();
                            }

                            // Set the tx_env and prepare to process it
                            evm.env.tx = tx;

                            // If the transaction is a state-changing transaction, `to_transact ==
                            // true` and the state will be written to the database via a
                            // `transact_commit()` Otherwise it must be
                            // a read-only call, so we will not update the database
                            // Calls will not have events to emit
                            if to_transact {
                                let execution_result = match evm.transact_commit() {
                                    // Check for an error in execution ([`EVMError<Infallible>`]),
                                    // but pass to the middleware to determine if the result is
                                    // [`ExecutionResult::Success`], [`ExecutionResult::Revert`], or
                                    // [`ExecutionResult::Halt`].
                                    Ok(val) => val,
                                    Err(e) => {
                                        state.store(
                                            State::Paused,
                                            std::sync::atomic::Ordering::SeqCst,
                                        );
                                        error!("Pausing the environment labeled {} due to an execution error: {:#?}", label, e);
                                        return Err(EnvironmentError::Execution(e));
                                    }
                                };
                                let event_broadcaster = event_broadcaster.lock().map_err(|e| {
                                    EnvironmentError::Communication(format!("{:?}", e))
                                })?;
                                event_broadcaster.broadcast(execution_result.logs())?;
                                let revm_result = RevmResult {
                                    outcome: TransactionOutcome::Success(execution_result),
                                    block_number: convert_uint_to_u64(evm.env.block.number)
                                        .map_err(|e| {
                                            EnvironmentError::Conversion(format!("{:?}", e))
                                        })?,
                                };
                                sender.send(revm_result).map_err(|e| {
                                    EnvironmentError::Communication(format!("{:?}", e))
                                })?;
                                counter += 1;
                            } else {
                                let result = match evm.transact() {
                                    // Check for an error in execution ([`EVMError<Infallible>`]),
                                    // but pass to the middleware to determine if the result is
                                    // [`ExecutionResult::Success`], [`ExecutionResult::Revert`], or
                                    // [`ExecutionResult::Halt`].
                                    Ok(result_and_state) => result_and_state.result,
                                    Err(e) => {
                                        state.store(
                                            State::Paused,
                                            std::sync::atomic::Ordering::SeqCst,
                                        );
                                        error!("Pausing the environment labeled {} due to an execution error: {:#?}", label, e);
                                        return Err(EnvironmentError::Execution(e));
                                    }
                                };
                                let result_and_block = RevmResult {
                                    outcome: TransactionOutcome::Success(result),
                                    block_number: convert_uint_to_u64(evm.env.block.number)
                                        .map_err(|e| {
                                            EnvironmentError::Conversion(format!("{:?}", e))
                                        })?,
                                };
                                sender.send(result_and_block).map_err(|e| {
                                    EnvironmentError::Communication(format!("{:?}", e))
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

/// Provides channels for communication between the EVM and external entities.
///
/// The socket contains senders and receivers for transactions, as well as an
/// event broadcaster to broadcast logs from the EVM to subscribers.
/// [`State`] is made atomic via the
/// `#[atomic_enum::atomic_enum](https://docs.rs/atomic_enum/latest/atomic_enum/)`
/// proc macro so that [`State`] can be loaded or stored from elsewhere (e.g.,
/// the [`Manager`]).
#[atomic_enum::atomic_enum]
#[derive(Eq, PartialEq)]
pub enum State {
    /// Upon constructing a new [`Environment`] the state will be
    /// [`State::Initialization`]. Modifications to the [`Environment`] can
    /// only whilst in this state.
    Initialization,

    /// Upon calling [`Environment::run()`] the [`Environment`]'s state will be
    /// set to [`State::Running`]. This means that the [`Environment`] is
    /// able to receive Ethereum calls/transactions via the `Socket` and
    /// should be echoing events via the [`EventBroadcaster`].
    Running,

    /// The [`Environment`] can be paused momentarily by manually with the
    /// [`Manager`] or by certain errors acting as triggers to pause.
    /// From the [`State::Paused`], the [`Environment`] can also be restarted
    /// (though one may have to resolve the error in place, if possible).
    Paused,

    /// If [`Environment`] is moved to [`State::Stopped`] it will shut down
    /// communication across the `Socket` and not be able to start again.
    /// This is either a sign that a simulation or process has successfully
    /// completed or that there was an error that was not recoverable or that
    /// the [`Manager`] called a stop manually.
    Stopped,
}

/// Provides channels for communication between the EVM and external entities.
///
/// The socket contains senders and receivers for transactions, as well as an
/// event broadcaster to broadcast logs from the EVM to subscribers.
#[derive(Debug, Clone)]
pub(crate) struct Socket {
    pub(crate) tx_sender: TxSender,
    pub(crate) tx_receiver: TxReceiver,
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
}

/// Represents the possible outcomes of an EVM transaction.
///
/// This enum is used to encapsulate both successful transaction results and
/// potential errors.
/// - `Success`: Indicates that the transaction was executed successfully and
///   contains the result of the execution. The wrapped `ExecutionResult`
///   provides detailed information about the transaction's execution, such as
///   returned values or changes made to the state.
/// - `Error`: Indicates that the transaction failed due to some error
///   condition. The wrapped `EnvironmentError` provides specifics about the
///   error, allowing callers to take appropriate action or relay more
///   informative error messages.
#[derive(Debug, Clone)]
pub(crate) enum TransactionOutcome {
    /// Represents a successfully executed transaction.
    ///
    /// Contains the result of the transaction's execution.
    Success(ExecutionResult),

    /// Represents a failed transaction due to some error.
    ///
    /// Contains information about the error that caused the transaction
    /// failure.
    Error(EnvironmentError),
}
/// Represents the result of an EVM transaction.
///
/// Contains the outcome of a transaction (e.g., success, revert, halt) and the
/// block number at which the transaction was executed.
#[derive(Debug, Clone)]
pub(crate) struct RevmResult {
    pub(crate) outcome: TransactionOutcome,
    pub(crate) block_number: U64,
}

/// Responsible for broadcasting Ethereum logs to subscribers.
///
/// Maintains a list of senders to which logs are sent whenever they are
/// produced by the EVM.
#[derive(Clone, Debug)]
pub(crate) struct EventBroadcaster(Vec<EventSender>);

impl EventBroadcaster {
    /// Called only when creating a new [`Environment`]
    fn new() -> Self {
        Self(vec![])
    }

    /// Called from [`RevmMiddleware`] implementation when setting up a new
    /// `FilterWatcher` as each watcher will need their own sender
    pub(crate) fn add_sender(&mut self, sender: EventSender) {
        self.0.push(sender);
    }

    /// Loop through each sender and send  `Vec<Log>` emitted from a transaction
    /// downstream to any and all receivers
    fn broadcast(&self, logs: Vec<Log>) -> Result<(), EnvironmentError> {
        for sender in &self.0 {
            sender
                .send(logs.clone())
                .map_err(|e| EnvironmentError::Communication(format!("{:?}", e)))?;
        }
        Ok(())
    }
}

/// Convert a U256 to a U64, discarding the higher bits if the number is larger
/// than 2^64 # Arguments
/// * `input` - The U256 to convert.
/// # Returns
/// * `Ok(U64)` - The converted U64.
/// Used for block number which is a U64.
#[inline]
fn convert_uint_to_u64(input: U256) -> Result<U64, &'static str> {
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
        let environment = Environment::new(TEST_ENV_LABEL.to_string(), 1.0, 1);
        assert_eq!(environment.label, TEST_ENV_LABEL);
        let state = environment.state.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(state, State::Initialization);
    }

    #[test]
    fn run() {
        let mut environment = Environment::new(TEST_ENV_LABEL.to_string(), 1.0, 1);
        environment.run();
        let state = environment.state.load(std::sync::atomic::Ordering::SeqCst);
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
