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
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::math::SeededPoisson;
#[cfg_attr(doc, doc(hidden))]
#[cfg_attr(doc, allow(unused_imports))]
#[cfg(doc)]
use crate::{manager::Manager, middleware::RevmMiddleware};

/// Alias for the sender of the channel for transmitting transactions.
pub(crate) type InstructionSender = Sender<Instruction>;

/// Alias for the receiver of the channel for transmitting transactions.
pub(crate) type InstructionReceiver = Receiver<Instruction>;

/// Alias for the sender of the channel for transmitting [`RevmResult`] emitted
/// from transactions.
pub(crate) type OutcomeSender = Sender<Result<Outcome, EnvironmentError>>;

/// Alias for the receiver of the channel for transmitting [`RevmResult`]
/// emitted from transactions.
pub(crate) type OutcomeReceiver = Receiver<Result<Outcome, EnvironmentError>>;

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
    pub parameters: EnvironmentParameters,

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

/// Parameters necessary for creating or modifying an `Environment`.
///
/// This structure holds configuration details or other parameters that might
/// be required when instantiating or updating an `Environment`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnvironmentParameters {
    /// A label for the [`Environment`].
    /// Used to allow the [`Manager`] to locate the [`Environment`] in order to
    /// control it. Also used to be able to organize, track progress, and
    /// post-process results.
    pub label: String,

    /// The type of block that will be used to step forward the [`EVM`].
    /// This can either be a [`BlockType::UserControlled`] or a
    /// [`BlockType::RandomlySampled`].
    /// The former will allow the end user to control the block number from
    /// their own external API and the latter will allow the end user to set
    /// a rate parameter and seed for a Poisson distribution that will be
    /// used to sample the amount of transactions per block.
    pub block_type: BlockType,
}

/// Allow the end user to be able to access a debug printout for the
/// [`Environment`]. Note that the [`EVM`] does not implement debug display,
/// hence the implementation by hand here.
impl Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Environment")
            .field("parameters", &self.parameters)
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
    #[error("error pausing! due to: {0:?}")]
    Pause(String),

    /// [`EnvironmentError::Communication`] is thrown when a channel for
    /// receiving or broadcasting fails in some way. This error could happen
    /// due to a channel being closed accidentally. If this is thrown, a
    /// restart of the simulation and an investigation into what caused a
    /// dropped channel is necessary.
    #[error("error communicating! due to: {0}")]
    Communication(String),

    /// [`EnvironmentError::Broadcast`] is thrown when the
    /// [`EventBroadcaster`] fails to broadcast events. This should be
    /// rare (if not impossible). If this is thrown, please report this error!
    #[error("error broadcasting! the source error is: {0}")]
    Broadcast(#[from] crossbeam_channel::SendError<Vec<Log>>),

    /// [`EnvironmentError::Conversion`] is thrown when a type fails to
    /// convert into another (typically a type used in `revm` versus a type used
    /// in [`ethers-rs`](https://github.com/gakonst/ethers-rs)).
    /// This error should be rare (if not impossible).
    /// Furthermore, after a switch to [`alloy`](https://github.com/alloy-rs)
    /// this will be (hopefully) unnecessary!
    #[error("conversion error! the source error is: {0}")]
    Conversion(String),

    /// [`EnvironmentError::TransactionReceivedWhilePaused`] is thrown when
    /// a transaction is received while the [`Environment`] is paused.
    /// This can be quite common due to concurrency issues, but should be
    /// handled gracefully.
    #[error("transaction was received while the environment was paused. this transaction was not processed.")]
    TransactionReceivedWhilePaused,

    /// [`EnvironmentError::NotUserControlledBlockType`] is thrown when
    /// the [`Environment`] is in a [`BlockType::RandomlySampled`] state and
    /// an attempt is made to externally change the block number and timestamp.
    #[error("error in the environment! attempted to externally change block number and timestamp when block type is not user controlled.")]
    NotUserControlledBlockType,
}

impl Environment {
    /// Privately accessible constructor function for creating an
    /// [`Environment`]. This function should be accessed by the
    /// [`Manager`].
    pub(crate) fn new(environment_parameters: EnvironmentParameters) -> Self {
        // Initialize the EVM used
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);

        // Chooose extra large code size and gas limit
        evm.env.cfg.limit_contract_code_size = Some(0x100000);
        evm.env.block.gas_limit = U256::MAX;

        let (instruction_sender, instruction_receiver) = unbounded();
        let socket = Socket {
            instruction_sender,
            instruction_receiver,
            event_broadcaster: Arc::new(Mutex::new(EventBroadcaster::new())),
        };

        Self {
            parameters: environment_parameters,
            state: Arc::new(AtomicState::new(State::Initialization)),
            evm,
            socket,
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
        let mut evm = self.evm.clone();
        let instruction_receiver = self.socket.instruction_receiver.clone();
        let event_broadcaster = self.socket.event_broadcaster.clone();
        let block_type = self.parameters.block_type.clone();
        let seeded_poisson = match block_type {
            BlockType::RandomlySampled {
                block_rate,
                block_time,
                seed,
            } => Some(SeededPoisson::new(block_rate, block_time, seed)),
            BlockType::UserControlled => None,
        };

        // Set up the state and tx counter
        self.state
            .store(State::Running, std::sync::atomic::Ordering::SeqCst);
        let state = Arc::clone(&self.state);
        let pausevar = Arc::clone(&self.pausevar);

        // Move the EVM and its socket to a new thread and retrieve this handle
        let handle = thread::spawn(move || {
            // Get the first amount of transactions per block from the distribution and set
            // the initial counter.
            let mut transactions_per_block = seeded_poisson
                .clone()
                .map(|mut distribution| distribution.sample());
            let mut transaction_index: usize = 0;
            let mut cumulative_gas_per_block: U256 = U256::ZERO;

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
                            .map_err(|e| EnvironmentError::Pause(e.to_string()))?;

                        // This logic here ensures we catch any last transactions and send
                        // the appropriate error so that we dont hang on the `tx_receiver`
                        while let Ok(request) = instruction_receiver.try_recv() {
                            let sender = match request {
                                Instruction::BlockUpdate { outcome_sender, .. } => outcome_sender,
                                Instruction::Call { outcome_sender, .. } => outcome_sender,
                                Instruction::Transaction { outcome_sender, .. } => outcome_sender,
                            };
                            sender
                                .send(Err(EnvironmentError::TransactionReceivedWhilePaused))
                                .map_err(|e| EnvironmentError::Communication(e.to_string()))?;
                        }

                        while state.load(std::sync::atomic::Ordering::SeqCst) == State::Paused {
                            guard = cvar
                                .wait(guard)
                                .map_err(|e| EnvironmentError::Pause(e.to_string()))?;
                        }
                    }

                    // Receive new transactions
                    State::Running => {
                        if let Ok(request) = instruction_receiver.try_recv() {
                            match request {
                                Instruction::BlockUpdate {
                                    block_number,
                                    block_timestamp,
                                    outcome_sender,
                                } => {
                                    if block_type != BlockType::UserControlled {
                                        outcome_sender
                                            .send(Err(EnvironmentError::NotUserControlledBlockType))
                                            .map_err(|e| {
                                                EnvironmentError::Communication(e.to_string())
                                            })?;
                                    }
                                    // Update the block number and timestamp
                                    evm.env.block.number = block_number;
                                    evm.env.block.timestamp = block_timestamp;
                                    transaction_index = 0;
                                    cumulative_gas_per_block = U256::ZERO;

                                    let receipt_data = ReceiptData {
                                        block_number: convert_uint_to_u64(evm.env.block.number)
                                            .unwrap(),
                                        transaction_index: U64::from(0), /* replace with actual
                                                                          * value */
                                        cumulative_gas_per_block: U256::from(0),
                                    };
                                    outcome_sender
                                        .send(Ok(Outcome::BlockUpdateCompleted(receipt_data)))
                                        .map_err(|e| {
                                            EnvironmentError::Communication(e.to_string())
                                        })?;
                                }
                                // A `Call` is not state changing and will not create events.
                                Instruction::Call {
                                    tx_env,
                                    outcome_sender,
                                } => {
                                    // Set the tx_env and prepare to process it
                                    evm.env.tx = tx_env;

                                    let result =
                                        evm.transact().map_err(EnvironmentError::Execution)?.result;
                                    outcome_sender
                                        .send(Ok(Outcome::CallCompleted(result)))
                                        .map_err(|e| {
                                            EnvironmentError::Communication(e.to_string())
                                        })?;
                                }
                                // A `Transaction` is state changing and will create events.
                                Instruction::Transaction {
                                    tx_env,
                                    outcome_sender,
                                } => {
                                    // Check whether we need to increment the block number given the
                                    // amount of transactions
                                    // that have occured on the current block and increment
                                    // if need be and draw a new sample from the `SeededPoisson`
                                    // distribution. Only do so if there is a distribution in the
                                    // first place.
                                    if transactions_per_block
                                        .is_some_and(|x| x == transaction_index)
                                    {
                                        transaction_index = 0;
                                        evm.env.block.number += U256::from(1);

                                        // This unwrap cannot fail.
                                        let mut seeded_poisson = seeded_poisson.clone().unwrap();

                                        evm.env.block.timestamp +=
                                            U256::from(seeded_poisson.time_step);
                                        transactions_per_block = Some(seeded_poisson.sample());
                                    }

                                    // Set the tx_env and prepare to process it
                                    evm.env.tx = tx_env;

                                    let block_number = convert_uint_to_u64(evm.env.block.number)?;
                                    let execution_result = evm
                                        .transact_commit()
                                        .map_err(EnvironmentError::Execution)?;

                                    // increment culmulative gas per block
                                    cumulative_gas_per_block +=
                                        U256::from(execution_result.clone().gas_used());

                                    let event_broadcaster =
                                        event_broadcaster.lock().map_err(|e| {
                                            EnvironmentError::Communication(e.to_string())
                                        })?;
                                    let receipt_data = ReceiptData {
                                        block_number,
                                        transaction_index: transaction_index.into(),
                                        cumulative_gas_per_block,
                                    };
                                    event_broadcaster.broadcast(execution_result.logs())?;
                                    outcome_sender
                                        .send(Ok(Outcome::TransactionCompleted(
                                            execution_result,
                                            receipt_data,
                                        )))
                                        .map_err(|e| {
                                            EnvironmentError::Communication(e.to_string())
                                        })?;
                                    transaction_index += 1;
                                }
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

/// [`Instruction`]s that can be sent to the [`Environment`] via the
/// [`Socket`].
/// These instructions can be `Call`s, `Transaction`s, or `BlockUpdate`s.
pub enum Instruction {
    /// A `BlockUpdate` is used to update the block number and timestamp of the
    /// [`EVM`].
    BlockUpdate {
        /// The block number to update the [`EVM`] to.
        block_number: U256,

        /// The block timestamp to update the [`EVM`] to.
        block_timestamp: U256,

        /// The sender used to to send the outcome of the block update back to.
        outcome_sender: OutcomeSender,
    },

    /// A `Call` is processed by the [`EVM`] but will not be state changing and
    /// will not create events.
    Call {
        /// The transaction environment for the call.
        tx_env: TxEnv,

        /// The sender used to to send the outcome of the call back to.
        outcome_sender: OutcomeSender,
    },

    /// A `Transaction` is processed by the [`EVM`] and will be state changing
    /// and will create events.
    Transaction {
        /// The transaction environment for the transaction.
        tx_env: TxEnv,

        /// The sender used to to send the outcome of the transaction back to.
        outcome_sender: OutcomeSender,
    },
}

/// [`RecieptData`] is a structure that holds the block number, transaction
/// index, and cumulative gas used per block for a transaction.
pub struct ReceiptData {
    /// `block_number` is the number of the block in which the transaction was
    /// included.
    pub(crate) block_number: U64,
    /// `transaction_index` is the index position of the transaction in the
    /// block.
    pub(crate) transaction_index: U64,
    /// [`cumulative_gas_per_block`] is the total amount of gas used in the
    /// block up until and including the transaction.
    pub(crate) cumulative_gas_per_block: U256,
}

/// [`Outcome`]s that can be sent back to the the client via the
/// [`Socket`].
/// These outcomes can be from `Call`, `Transaction`, or `BlockUpdate`
/// instructions sent to the [`Environment`]
pub enum Outcome {
    /// The outcome of a `BlockUpdate` instruction that is used to provide a
    /// non-error output of updating the block number and timestamp of the
    /// [`EVM`] to the client.
    BlockUpdateCompleted(ReceiptData),

    /// The outcome of a `Call` instruction that is used to provide the output
    /// of some [`EVM`] computation to the client.
    CallCompleted(ExecutionResult),

    // TODO: This top one should probably be a tuple variant that has any extra necessary stuff to
    // form a receipt so long as the transaction was successful
    /// The outcome of a `Transaction` instruction that is first unpacked to see
    /// if the result is successful, then it can be used to build a
    /// `TransactionReceipt` in the `Middleware`.
    TransactionCompleted(ExecutionResult, ReceiptData),
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
    pub(crate) instruction_sender: InstructionSender,
    pub(crate) instruction_receiver: InstructionReceiver,
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
}

/// Provides a means of deciding how the block number of the [`EVM`] will be
/// chosen.
/// This can either be a [`BlockType::UserControlled`] or a
/// [`BlockType::RandomlySampled`].
/// The former will allow the end user to control the block number from
/// their own external API and the latter will allow the end user to set
/// a rate parameter and seed for a Poisson distribution that will be
/// used to sample the amount of transactions per block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlockType {
    /// The block number will be controlled by the end user.
    UserControlled,

    /// The block number will be sampled from a Poisson distribution.
    /// A seeded Poisson distribution that is sampled from in order to determine
    /// the average block size. [`SeededPoisson`] is created with a seed in
    /// order to have repeatable simulations.
    RandomlySampled {
        /// The mean of the rate at which the environment will
        /// process blocks (e.g., the rate parameter in the Poisson distribution
        /// used in the [`SeededPoisson`] field of an [`Environment`]).
        block_rate: f64,

        /// The amount of time the block timestamp will increase for each new
        /// block.
        block_time: u32,

        /// A value chosen to generate randomly chosen block sizes
        /// for the environment.
        seed: u64,
    },
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
            sender.send(logs.clone())?;
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
fn convert_uint_to_u64(input: U256) -> Result<U64, EnvironmentError> {
    let as_str = input.to_string();
    match as_str.parse::<u64>() {
        Ok(val) => Ok(val.into()),
        Err(_) => Err(EnvironmentError::Conversion(
            "U256 value is too large to fit into u64".to_string(),
        )),
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;

    pub(crate) const TEST_ENV_LABEL: &str = "test";

    #[test]
    fn new_user_controlled() {
        let params = EnvironmentParameters {
            label: TEST_ENV_LABEL.to_string(),
            block_type: BlockType::UserControlled,
        };
        let environment = Environment::new(params);
        assert_eq!(environment.parameters.label, TEST_ENV_LABEL);
        let state = environment.state.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(state, State::Initialization);
    }

    #[test]
    fn new_randomly_sampled() {
        let block_type = BlockType::RandomlySampled {
            block_rate: 1.0,
            block_time: 12,
            seed: 1,
        };
        let params = EnvironmentParameters {
            label: TEST_ENV_LABEL.to_string(),
            block_type,
        };
        let environment = Environment::new(params);
        assert_eq!(environment.parameters.label, TEST_ENV_LABEL);
        let state = environment.state.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(state, State::Initialization);
    }

    #[test]
    fn run() {
        let params = EnvironmentParameters {
            label: TEST_ENV_LABEL.to_string(),
            block_type: BlockType::UserControlled,
        };
        let mut environment = Environment::new(params);
        environment.run();
        let state = environment.state.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(state, State::Running);
    }

    #[test]
    fn conversion() {
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
