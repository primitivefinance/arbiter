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
use log::error;
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

/// Parameters necessary for creating or modifying an `Environment`.
///
/// This structure holds configuration details or other parameters that might
/// be required when instantiating or updating an `Environment`.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct EnvironmentParameters {
    /// A label for the [`Environment`].
    /// Used to allow the [`Manager`] to locate the [`Environment`] in order to
    /// control it. Also used to be able to organize, track progress, and
    /// post-process results.
    pub label: Option<String>,

    /// The type of block that will be used to step forward the [`EVM`].
    /// This can either be a [`BlockType::UserControlled`] or a
    /// [`BlockType::RandomlySampled`].
    /// The former will allow the end user to control the block number from
    /// their own external API and the latter will allow the end user to set
    /// a rate parameter and seed for a Poisson distribution that will be
    /// used to sample the amount of transactions per block.
    pub block_settings: BlockSettings,

    /// The gas settings for the [`Environment`].
    /// This can either be [`GasSettings::UserControlled`],
    /// [`GasSettings::RandomlySampled`], or [`GasSettings::Constant`].
    /// The first will allow the end user to control the gas price from
    /// their own external API (not yet implemented) and the second will allow
    /// the end user to set a multiplier for the gas price that will be used
    /// to sample the amount of transactions per block. The last will allow
    /// the end user to set a constant gas price for all transactions.
    /// By default, [`GasSettings::UserControlled`] begins with a gas price of
    /// 0.
    pub gas_settings: GasSettings,
}

pub struct EnvironmentBuilder {
    pub label: Option<String>,
    pub block_settings: BlockSettings,
    pub gas_settings: GasSettings
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
/// ## What are we trying to catch?
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

    /// [`EnvironmentError::Transaction`] is thrown when a transaction fails
    /// to be processed by the [`EVM`]. This could be due to a insufficient
    /// funds to pay for gas, an invalid nonce, or other reasons. This error
    /// can be quite common and should be handled gracefully.
    #[error("transaction error! the source error is: {0:?}")]
    Transaction(InvalidTransaction),

    /// [`EnvironmentError::Account`] is thrown when there is an issue handling
    /// accounts in the [`EVM`]. This could be due to an account already
    /// existing or other reasons.
    #[error("account error! due to: {0:?}")]
    Account(String),

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

    /// [`EnvironmentError::NotUserControlledGasSettings`] is thrown when the
    /// [`Environment`] is not in a [`GasSettings::UserControlled`] state and
    /// an attempt is made to externally change the gas price.
    #[error("error in the environment! attempted to set a gas price when the `GasSettings` is not `GasSettings::UserControlled`")]
    NotUserControlledGasSettings,

    /// [`EnvironmentError::NotUserControlledBlockType`] is thrown when
    /// the [`Environment`] is in a [`BlockType::RandomlySampled`] state and
    /// an attempt is made to externally change the block number and timestamp.
    #[error("error in the environment! attempted to externally change block number and timestamp when `BlockType` is not `BlockType::UserControlled`.")]
    NotUserControlledBlockType,

    /// [`EnvironmentError::NotRandomlySampledBlockType`] is thrown when
    /// the [`Environment`] is **not** in a [`BlockType::RandomlySampled`] state
    /// and an attempt is made to set the gas price via a multiplier.
    /// That is, the user has chosen [`GasSettings::RandomlySampled`] without
    /// [`BlockType::RandomlySampled`].
    #[error("error in the environment! attempted to set a gas price via a multiplier when the `BlockType` is not `BlockType::RandomlySampled`.")]
    NotRandomlySampledBlockType,
}

/// The `EnvironmentBuilder` is a builder pattern for creating an [`Environment`].
/// It allows for the configuration of the [`Environment`] before it is created.
impl EnvironmentBuilder {
    /// Creates a new `EnvironmentBuilder` with default settings.
    /// By default, the `block_settings` and `gas_settings` are set to `UserControlled`.
    pub fn new() -> Self {
        Self {
            label: None,
            block_settings: BlockSettings::UserControlled,
            gas_settings: GasSettings::UserControlled
        }
    }

    /// Sets the `block_settings` for the `EnvironmentBuilder`.
    /// This determines how the block number and timestamp are controlled in the [`Environment`].
    pub fn block_settings(mut self, block_settings: BlockSettings) -> Self {
        self.block_settings = block_settings;
        self
    }

    /// Sets the `gas_settings` for the `EnvironmentBuilder`.
    /// This determines how the gas price is controlled in the [`Environment`].
    pub fn gas_settings(mut self, gas_settings: GasSettings) -> Self {
        self.gas_settings = gas_settings;
        self
    }

    /// Sets the `label` for the `EnvironmentBuilder`.
    /// This is an optional string that can be used to identify the [`Environment`].
    pub fn label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Converts the `EnvironmentBuilder` into `EnvironmentParameters`.
    /// This is a private function used in the `build` function.
    fn into_environment_parameters(&self) -> EnvironmentParameters {
        EnvironmentParameters {
            label: self.label.clone(),
            block_settings: self.block_settings.clone(),
            gas_settings: self.gas_settings.clone()
        }
    }

    /// Builds the `Environment` from the `EnvironmentBuilder`.
    /// This consumes the `EnvironmentBuilder` and returns an [`Environment`].
    pub fn build(self) -> Environment {
        let parameters = self.into_environment_parameters();
        Environment::new(parameters)
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
    pub(crate) fn run(&mut self) {
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
}

/// [`Instruction`]s that can be sent to the [`Environment`] via the
/// [`Socket`].
/// These instructions can be:
/// - [`Instruction::AddAccount`],
/// - [`Instruction::BlockUpdate`],
/// - [`Instruction::Call`],
/// - [`Instruction::Transaction`],
/// - [`Instruction::Query`].
/// The [`Instruction`]s are sent to the [`Environment`] via the
/// [`Socket::instruction_sender`] and the results are received via the
/// [`crate::middleware::Connection::outcome_receiver`].
pub enum Instruction {
    /// An `AddAccount` is used to add a default/unfunded account to the
    /// [`EVM`].
    AddAccount {
        /// The address of the account to add to the [`EVM`].
        address: ethers::types::Address,

        /// The sender used to to send the outcome of the account addition back
        /// to.
        outcome_sender: OutcomeSender,
    },

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

    /// A `Deal` is used to increase the balance of an account in the [`EVM`].
    Deal {
        /// The address of the account to increase the balance of.
        address: ethers::types::Address,

        /// The amount to increase the balance of the account by.
        amount: ethers::types::U256,

        /// The sender used to to send the outcome of the deal back to.
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

    /// A `SetGasPrice` is used to set the gas price of the [`EVM`].
    SetGasPrice {
        /// The gas price to set the [`EVM`] to.
        gas_price: ethers::types::U256,

        /// The sender used to to send the outcome of the gas price setting back
        /// to.
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

    /// A `Query` is used to query the [`EVM`] for some data, the choice of
    /// which data is specified by the inner `EnvironmentData` enum.
    Query {
        /// The data to query the [`EVM`] for.
        environment_data: EnvironmentData,

        /// The sender used to to send the outcome of the query back to.
        outcome_sender: OutcomeSender,
    },
}

/// [`EnvironmentData`] is an enum used inside of the [`Instruction::Query`] to
/// specify what data should be returned to the user.
/// Currently this may be the block number, block timestamp, gas price, or
/// balance of an account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnvironmentData {
    /// The query is for the block number of the [`EVM`].
    BlockNumber,

    /// The query is for the block timestamp of the [`EVM`].
    BlockTimestamp,

    /// The query is for the gas price of the [`EVM`].
    GasPrice,

    /// The query is for the balance of an account given by the inner `Address`.
    Balance(ethers::types::Address),
}

/// [`ReceiptData`] is a structure that holds the block number, transaction
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
    /// The outcome of an [`Instruction::AddAccount`] instruction that is used
    /// to signify that the account was added successfully.
    AddAccountCompleted,

    /// The outcome of a `BlockUpdate` instruction that is used to provide a
    /// non-error output of updating the block number and timestamp of the
    /// [`EVM`] to the client.
    BlockUpdateCompleted(ReceiptData),

    /// The outcome of a [`Instruction::Deal`] instruction that is used to
    /// signify that increasing the balance of an account was successful.
    DealCompleted,

    /// The outcome of a `Call` instruction that is used to provide the output
    /// of some [`EVM`] computation to the client.
    CallCompleted(ExecutionResult),

    /// The outcome of a [`Instruction::SetGasPrice`] instruction that is used
    /// to signify that the gas price was set successfully.
    SetGasPriceCompleted,

    /// The outcome of a `Transaction` instruction that is first unpacked to see
    /// if the result is successful, then it can be used to build a
    /// `TransactionReceipt` in the `Middleware`.
    TransactionCompleted(ExecutionResult, ReceiptData),

    /// The outcome of a `Query` instruction that carries a `String`
    /// representation of the data. Currently this may carry the block
    /// number, block timestamp, gas price, or balance of an account.
    QueryReturn(String),
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BlockSettings {
    /// The block number will be controlled by the end user.
    #[default]
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

/// Provides a means of deciding how the gas price of the
/// [`EVM`] will be chosen.
/// This can either be a [`GasSettings::UserControlled`],
/// [`GasSettings::RandomlySampled`], or [`GasSettings::None`].
/// The former will allow the end user to control the gas price from
/// their own external API and the latter will allow the end user to set
/// a constant gas price.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum GasSettings {
    /// The gas limit will be controlled by the end user.
    /// In the future, Foundry cheatcodes will be used to control gas
    /// on-the-fly.
    #[default]
    UserControlled,

    /// The gas price will depend on the number of transactions in the block.
    /// The user *must* set the [`BlockType`] to [`BlockType::RandomlySampled`].
    /// We determine the gas price by multiplying the number of transactions in
    /// the block by the multiplier which represents paying higher fees for a
    /// more congested network.
    RandomlySampled {
        /// Multiplies the number of transactions in the block to determine the
        /// gas price.
        multiplier: f64,
    },

    /// The gas price will be a constant value from the inner value.
    Constant(u128),
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
    fn new_with_builder() {
        let environment = EnvironmentBuilder::new().build();
        assert_eq!(environment.parameters.label, None);
        let state = environment.state.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(state, State::Initialization);
    }
    #[test]
    fn new_with_builder_custom_settings() {
        let environment = EnvironmentBuilder::new()
            .label(TEST_ENV_LABEL.into())
            .block_settings(BlockSettings::RandomlySampled {
                block_rate: 1.0,
                block_time: 12,
                seed: 1,
            })
            .gas_settings(GasSettings::RandomlySampled { multiplier: 1.0 })
            .build();
        assert_eq!(environment.parameters.label, Some(TEST_ENV_LABEL.into()));
        let state = environment.state.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(state, State::Initialization);
    }
    #[test]
    fn new_user_controlled() {
        let params = EnvironmentParameters {
            label: Some(TEST_ENV_LABEL.to_string()),
            block_settings: BlockSettings::UserControlled,
            gas_settings: GasSettings::UserControlled,
        };
        let environment = Environment::new(params);
        assert_eq!(environment.parameters.label, Some(TEST_ENV_LABEL.into()));
        let state = environment.state.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(state, State::Initialization);
    }

    #[test]
    fn new_randomly_sampled() {
        let block_type = BlockSettings::RandomlySampled {
            block_rate: 1.0,
            block_time: 12,
            seed: 1,
        };
        let params = EnvironmentParameters {
            label: Some(TEST_ENV_LABEL.to_string()),
            block_settings: block_type,
            gas_settings: GasSettings::RandomlySampled { multiplier: 1.0 },
        };
        let environment = Environment::new(params);
        assert_eq!(environment.parameters.label, Some(TEST_ENV_LABEL.into()));
        let state = environment.state.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(state, State::Initialization);
    }

    #[test]
    fn run() {
        let params = EnvironmentParameters {
            label: Some(TEST_ENV_LABEL.to_string()),
            block_settings: BlockSettings::UserControlled,
            gas_settings: GasSettings::UserControlled,
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
