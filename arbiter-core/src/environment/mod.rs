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
//! - `EnvironmentParameters`: Parameters necessary for creating or modifying
//!  an `Environment`.
//!     - `BlockSettings`: Enum indicating how block numbers and timestamps are
//!       moved forward.
//!     - `GasSettings`: Enum indicating the type of gas settings that will be
//!     used to make clients pay gas.
//! - `Instruction`: Enum indicating the type of instruction that is being sent
//!  to the EVM.
//! - `Outcome`: Enum indicating the type of outcome that is being sent back
//!  from the EVM.
//! - `EnvironmentError`: Enum indicating the type of error that can be thrown
//!  by the EVM.
//! - `State`: Enum indicating the current state of the environment.
//! - `Socket`: Provides channels for communication between the EVM and the
//!   outside world.
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
use log::{error, info};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{
        AccountInfo, EVMError, ExecutionResult, HashMap, InvalidTransaction, Log, TxEnv, U256,
    },
    EVM,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::math::SeededPoisson;
#[cfg_attr(doc, doc(hidden))]
#[cfg_attr(doc, allow(unused_imports))]
#[cfg(doc)]
use crate::{manager::Manager, middleware::RevmMiddleware};

pub(crate) mod instruction;
use instruction::*;

pub mod error;
use error::*;

pub mod builder;
use builder::*;

#[cfg(test)]
pub(crate) mod tests;

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
/// parameter, typically denoted by the Greek letter lambda, and set this to be
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

impl Environment {
    /// Privately accessible constructor function for creating an
    /// [`Environment`]. This function should be accessed by the
    /// [`Manager`].
    pub(crate) fn new(environment_parameters: EnvironmentParameters) -> Self {
        // Initialize the EVM used
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);

        // Choose extra large code size and gas limit
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
    pub fn run(&mut self) {
        // Pull clones of the relevant data prepare to send into a new thread
        let mut evm = self.evm.clone();
        let instruction_receiver = self.socket.instruction_receiver.clone();
        let event_broadcaster = self.socket.event_broadcaster.clone();
        let block_type = self.parameters.block_settings.clone();
        let seeded_poisson = match block_type {
            BlockSettings::RandomlySampled {
                block_rate,
                block_time,
                seed,
            } => Some(Arc::new(Mutex::new(SeededPoisson::new(
                block_rate, block_time, seed,
            )))),
            BlockSettings::UserControlled => None,
        };
        let gas_settings = self.parameters.gas_settings.clone();

        // Set up the state and tx counter
        self.state
            .store(State::Running, std::sync::atomic::Ordering::SeqCst);
        let state = Arc::clone(&self.state);
        let pausevar = Arc::clone(&self.pausevar);

        // Move the EVM and its socket to a new thread and retrieve this handle
        let handle = thread::spawn(move || {
            if let GasSettings::RandomlySampled { multiplier: _ } = gas_settings {
                if seeded_poisson.is_none() {
                    return Err(EnvironmentError::NotRandomlySampledBlockType);
                }
            }
            // Get the first amount of transactions per block from the distribution and set
            // the initial counter.
            let mut transactions_per_block = seeded_poisson
                .clone()
                .map(|distribution| distribution.lock().unwrap().sample());
            match gas_settings {
                GasSettings::UserControlled => {
                    evm.env.tx.gas_price = U256::from(0);
                }
                GasSettings::RandomlySampled { multiplier } => {
                    let gas_price = (transactions_per_block
                        .ok_or(EnvironmentError::NotRandomlySampledBlockType)?
                        as f64)
                        * multiplier;
                    evm.env.tx.gas_price = U256::from(gas_price as u128);
                }
                GasSettings::Constant(gas_price) => {
                    evm.env.tx.gas_price = U256::from(gas_price);
                }
            }
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

                        // TODO: It may actually be okay to allow DB changes upon a pause. We should
                        // consider this. This logic here ensures we catch
                        // any last transactions and send the appropriate
                        // error so that we dont hang on the `tx_receiver`
                        while let Ok(request) = instruction_receiver.try_recv() {
                            let sender = match request {
                                Instruction::AddAccount { outcome_sender, .. } => outcome_sender,
                                Instruction::BlockUpdate { outcome_sender, .. } => outcome_sender,
                                Instruction::Deal { outcome_sender, .. } => outcome_sender,
                                Instruction::Call { outcome_sender, .. } => outcome_sender,
                                Instruction::SetGasPrice { outcome_sender, .. } => outcome_sender,
                                Instruction::Transaction { outcome_sender, .. } => outcome_sender,
                                Instruction::Query { outcome_sender, .. } => outcome_sender,
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
                                Instruction::AddAccount {
                                    address,
                                    outcome_sender,
                                } => {
                                    let db = evm.db.as_mut().unwrap();
                                    let recast_address = revm::primitives::Address::from(address);
                                    let account = revm::db::DbAccount {
                                        info: AccountInfo::default(),
                                        account_state: revm::db::AccountState::None,
                                        storage: HashMap::new(),
                                    };
                                    match db.accounts.insert(recast_address, account) {
                                        None => {
                                            outcome_sender
                                                .send(Ok(Outcome::AddAccountCompleted))
                                                .map_err(|e| {
                                                EnvironmentError::Communication(e.to_string())
                                            })?;
                                        }
                                        Some(_) => {
                                            outcome_sender
                                                .send(Err(EnvironmentError::Account(
                                                    "Account already exists!".to_string(),
                                                )))
                                                .map_err(|e| {
                                                    EnvironmentError::Communication(e.to_string())
                                                })?;
                                        }
                                    }
                                }
                                Instruction::BlockUpdate {
                                    block_number,
                                    block_timestamp,
                                    outcome_sender,
                                } => {
                                    if block_type != BlockSettings::UserControlled {
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
                                Instruction::Deal {
                                    address,
                                    amount,
                                    outcome_sender,
                                } => {
                                    let db = evm.db.as_mut().unwrap();
                                    let recast_address = revm::primitives::Address::from(address);
                                    match db.accounts.get_mut(&recast_address) {
                                        Some(account) => {
                                            account.info.balance += U256::from_limbs(amount.0);
                                            outcome_sender
                                                .send(Ok(Outcome::DealCompleted))
                                                .map_err(|e| {
                                                    EnvironmentError::Communication(e.to_string())
                                                })?;
                                        }
                                        None => {
                                            outcome_sender
                                                .send(Err(EnvironmentError::Account(
                                                    "Account is missing!".to_string(),
                                                )))
                                                .map_err(|e| {
                                                    EnvironmentError::Communication(e.to_string())
                                                })?;
                                        }
                                    };
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
                                Instruction::SetGasPrice {
                                    gas_price,
                                    outcome_sender,
                                } => {
                                    if GasSettings::UserControlled != gas_settings {
                                        outcome_sender
                                            .send(Err(
                                                EnvironmentError::NotUserControlledGasSettings,
                                            ))
                                            .map_err(|e| {
                                                EnvironmentError::Communication(e.to_string())
                                            })?;
                                    }
                                    evm.env.tx.gas_price = U256::from_limbs(gas_price.0);
                                    outcome_sender
                                        .send(Ok(Outcome::SetGasPriceCompleted))
                                        .map_err(|e| {
                                            EnvironmentError::Communication(e.to_string())
                                        })?;
                                }

                                // A `Transaction` is state changing and will create events.
                                Instruction::Transaction {
                                    tx_env,
                                    outcome_sender,
                                } => {
                                    // Set the tx_env and prepare to process it
                                    evm.env.tx = tx_env;

                                    let execution_result = match evm
                                        .inspect_commit(revm::inspectors::GasInspector::default())
                                    {
                                        Ok(result) => result,
                                        Err(e) => {
                                            if let EVMError::Transaction(invalid_transaction) = e {
                                                outcome_sender
                                                    .send(Err(EnvironmentError::Transaction(
                                                        invalid_transaction,
                                                    )))
                                                    .map_err(|e| {
                                                        EnvironmentError::Communication(
                                                            e.to_string(),
                                                        )
                                                    })?;
                                                continue;
                                            } else {
                                                outcome_sender
                                                    .send(Err(EnvironmentError::Execution(e)))
                                                    .map_err(|e| {
                                                        EnvironmentError::Communication(
                                                            e.to_string(),
                                                        )
                                                    })?;
                                                continue;
                                            }
                                        }
                                    };
                                    let block_number = convert_uint_to_u64(evm.env.block.number)?;

                                    // increment cumulative gas per block
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

                                    // Check whether we need to increment the block number given the
                                    // amount of transactions
                                    // that have occurred on the current block and increment
                                    // if need be and draw a new sample from the `SeededPoisson`
                                    // distribution. Only do so if there is a distribution in the
                                    // first place.
                                    if transactions_per_block
                                        .is_some_and(|x| x == transaction_index)
                                    {
                                        transaction_index = 0;
                                        evm.env.block.number += U256::from(1);

                                        // This unwrap cannot fail.
                                        let seeded_poisson_clone = seeded_poisson.clone().unwrap();
                                        let mut seeded_poisson_lock =
                                            seeded_poisson_clone.lock().unwrap();

                                        evm.env.block.timestamp +=
                                            U256::from(seeded_poisson_lock.time_step);
                                        transactions_per_block = loop {
                                            let sample = Some(seeded_poisson_lock.sample());

                                            if sample == Some(0) {
                                                evm.env.block.number += U256::from(1);
                                                continue;
                                            } else {
                                                break sample;
                                            }
                                        };
                                        if let GasSettings::RandomlySampled { multiplier } =
                                            gas_settings
                                        {
                                            let gas_price = (transactions_per_block.ok_or(
                                                EnvironmentError::NotRandomlySampledBlockType,
                                            )?
                                                as f64)
                                                * multiplier;
                                            evm.env.tx.gas_price = U256::from(gas_price as u128);
                                        };
                                    }
                                }
                                Instruction::Query {
                                    environment_data,
                                    outcome_sender,
                                } => {
                                    let outcome = match environment_data {
                                        EnvironmentData::BlockNumber => Ok(Outcome::QueryReturn(
                                            evm.env.block.number.to_string(),
                                        )),
                                        EnvironmentData::BlockTimestamp => {
                                            Ok(Outcome::QueryReturn(
                                                evm.env.block.timestamp.to_string(),
                                            ))
                                        }
                                        EnvironmentData::GasPrice => Ok(Outcome::QueryReturn(
                                            evm.env.tx.gas_price.to_string(),
                                        )),
                                        EnvironmentData::Balance(address) => {
                                            // This unwrap should never fail.
                                            let db = evm.db().unwrap();
                                            let recast_address =
                                                revm::primitives::Address::from(address);
                                            match db.accounts.get(&recast_address) {
                                                Some(account) => Ok(Outcome::QueryReturn(
                                                    account.info.balance.to_string(),
                                                )),
                                                None => Err(EnvironmentError::Account(
                                                    "Account is missing!".to_string(),
                                                )),
                                            }
                                        }
                                    };
                                    outcome_sender.send(outcome).map_err(|e| {
                                        EnvironmentError::Communication(e.to_string())
                                    })?;
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

    /// Pauses the execution of the environment.
    ///
    /// This method changes the state of the environment to `Paused` if it is
    /// currently `Running`. If the environment is already `Paused`, it does
    /// nothing and returns `Ok(())`. If the environment is `Stopped`, it
    /// returns an `Err(EnvironmentError::Pause("Environment is stopped. Cannot
    /// pause."))`. If the environment is in `Initialization` state, it
    /// returns an `Err(EnvironmentError::Pause("Environment is in an invalid
    /// state: Initialization. This should not be possible."))`.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the environment was successfully paused or was already
    ///   paused.
    /// * `Err(EnvironmentError::Pause(String))` if the environment is stopped
    ///   or in an invalid state.
    pub fn pause(&mut self) -> Result<(), EnvironmentError> {
        match self.state.load(std::sync::atomic::Ordering::SeqCst) {
            State::Running => {
                self.state
                    .store(State::Paused, std::sync::atomic::Ordering::SeqCst);
                if let Some(label) = &self.parameters.label {
                    info!("Pausing environment with label: {}", label);
                } else {
                    info!("Pausing environment with no label.");
                }
                Ok(())
            }
            State::Paused => Ok(()),
            State::Stopped => Err(EnvironmentError::Pause(
                "Environment is stopped. Cannot pause.".into(),
            )),
            State::Initialization => Err(EnvironmentError::Pause(
                "Environment is in an invalid state: Initialization. This should not be possible."
                    .into(),
            )),
        }
    }

    /// Stops the execution of the environment.
    ///
    /// This method changes the state of the environment to `Stopped` if it is
    /// currently `Running` or `Paused`. If the environment is already
    /// `Stopped`, it does nothing and returns `Ok(())`. If the environment
    /// is in `Initialization` state, it returns an
    /// `Err(EnvironmentError::Stop("Environment is in an invalid state:
    /// Initialization. This should not be possible."))`.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the environment was successfully stopped or was already
    ///   stopped.
    /// * `Err(EnvironmentError::Stop(String))` if the environment is in an
    ///   invalid state.
    pub fn stop(&mut self) -> Result<(), EnvironmentError> {
        match self.state.load(std::sync::atomic::Ordering::SeqCst) {
            State::Running => {
                self.state
                    .store(State::Stopped, std::sync::atomic::Ordering::SeqCst);
                if let Some(label) = &self.parameters.label {
                    info!("Stopping environment with label: {}", label);
                } else {
                    info!("Stopping environment with no label.");
                }
                Ok(())
            }
            State::Paused => {
                self.state
                    .store(State::Stopped, std::sync::atomic::Ordering::SeqCst);
                info!(
                    "Stopping environment with label: {}",
                    self.parameters.label.as_ref().unwrap_or(&"".into())
                );
                Ok(())
            }
            State::Stopped => Ok(()),
            State::Initialization => Err(EnvironmentError::Stop(
                "Environment is in an invalid state: Initialization. This should not be possible."
                    .into(),
            )),
        }
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
    pub(crate) instruction_sender: InstructionSender,
    pub(crate) instruction_receiver: InstructionReceiver,
    pub(crate) event_broadcaster: Arc<Mutex<EventBroadcaster>>,
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
