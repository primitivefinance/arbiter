//! The `middleware` module provides functionality to interact with
//! Ethereum-like virtual machines. It achieves this by offering a middleware
//! implementation for sending and reading transactions, as well as watching
//! for events.
//!
//! Main components:
//! - [`RevmMiddleware`]: The core middleware implementation.
//! - [`RevmMiddlewareError`]: Error type for the middleware.
//! - [`Connection`]: Handles communication with the Ethereum VM.
//! - `FilterReceiver`: Facilitates event watching based on certain filters.

#![warn(missing_docs)]

use std::{
    collections::HashMap,
    fmt::Debug,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    time::Duration,
};

use ethers::{
    abi::ethereum_types::BloomInput,
    prelude::{
        k256::{
            ecdsa::SigningKey,
            sha2::{Digest, Sha256},
        },
        ProviderError,
    },
    providers::{
        FilterKind, FilterWatcher, JsonRpcClient, Middleware, MiddlewareError, PendingTransaction,
        Provider,
    },
    signers::{Signer, Wallet},
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockId, Bloom, Bytes, Filter,
        FilteredParams, Log, Transaction, TransactionReceipt, U64,
    },
};
use futures_timer::Delay;
use rand::{rngs::StdRng, SeedableRng};
use revm::primitives::{CreateScheme, ExecutionResult, Output, TransactTo, TxEnv, B160, U256};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::environment::{
    Environment, EventBroadcaster, Instruction, InstructionSender, Outcome, OutcomeReceiver,
    OutcomeSender, ReceiptData,
};

/// A middleware structure that integrates with `revm`.
///
/// [`RevmMiddleware`] serves as a bridge between the application and `revm`'s
/// execution environment, allowing for transaction sending, call execution, and
/// other core functions. It uses a custom connection and error system tailored
/// to Revm's specific needs.
///
/// This allows for `revm` and the [`Environment`] built around it to be treated
/// in much the same way as a live EVM blockchain can be addressed.
///
/// # Examples
///
/// Basic usage:
/// ```
/// // Get the necessary dependencies
/// // Import `Arc` if you need to create a client instance
/// use std::sync::Arc;
///
/// use arbiter_core::{
///     environment::{BlockType, EnvironmentParameters},
///     manager::Manager,
///     middleware::RevmMiddleware,
/// };
///
/// // Create a manager and add an environment
/// let mut manager = Manager::new();
/// let params = EnvironmentParameters {
///     label: "example_env".to_string(),
///     block_type: BlockType::RandomlySampled {
///         block_rate: 1.0,
///         block_time: 12,
///         seed: 1,
///     },
/// };
/// manager.add_environment(params).unwrap();
///
/// // Retrieve the environment to create a new middleware instance
/// let environment = manager.environments.get("example_env").unwrap();
/// let middleware = RevmMiddleware::new(&environment, Some("test_label".to_string()));
/// let client = Arc::new(&middleware);
/// ```
/// The client can now be used for transactions with the environment.
/// Use a seed like `Some("test_label".to_string())` for maintaining a
/// consistant address across simulations and client labeling. Seeding is be
/// useful for debugging and post-processing.
#[derive(Debug)]
pub struct RevmMiddleware {
    provider: Provider<Connection>,
    wallet: Wallet<SigningKey>,
}

/// Errors that can occur while using the [`RevmMiddleware`].
/// These errors are likely to be more common than other errors in
/// `arbiter-core` as they can come from simple issues such as contract reverts
/// or halts. Certain errors such as [`RevmMiddlewareError::Send`],
/// [`RevmMiddlewareError::Receive`], [`RevmMiddlewareError::Conversion`],
/// [`RevmMiddlewareError::Json`], and [`RevmMiddlewareError::EventBroadcaster`]
/// are considered more worrying. If these are achieved, please feel free to
/// contact our team via the [Telegram group](https://t.me/arbiter_rs) or on
/// [GitHub](https://github.com/primitivefinance/arbiter/).
#[derive(Error, Debug)]
pub enum RevmMiddlewareError {
    /// An error occurred while attempting to interact with the environment.
    #[error("an error came from the environment! due to: {0}")]
    Environment(#[from] crate::environment::EnvironmentError),

    /// An error occurred while attempting to send a transaction.
    #[error("failed to send transaction! due to: {0}")]
    Send(String),

    /// There was an issue receiving an [`ExecutionResult`], possibly from
    /// another service or module.
    #[error("failed to receive `ExecutionResult`! due to: {0}")]
    Receive(#[from] crossbeam_channel::RecvError),

    /// There was a failure trying to obtain a lock on the [`EventBroadcaster`],
    /// possibly due to concurrency issues.
    #[error("failed to gain event broadcaster lock! due to: {0}")]
    EventBroadcaster(String),

    /// The required data for a transaction was missing or incomplete.
    #[error("missing data! due to: {0}")]
    MissingData(String),

    /// An error occurred during type conversion, possibly when translating
    /// between domain-specific types.
    #[error("failed to convert types! due to: {0}")]
    Conversion(String),

    /// An error occurred while trying to serialize or deserialize JSON data.
    #[error("failed to handle with JSON data! due to: {0:?}")]
    Json(serde_json::Error),

    /// The execution of a transaction was reverted, indicating that the
    /// transaction was not successful.
    #[error("execution failed to succeed due to revert!\n gas used is: {gas_used}\n output is {output:?}")]
    ExecutionRevert {
        /// Provides the amount of gas used by the transaction.
        gas_used: u64,

        /// Provides the output or reason why the transaction was reverted.
        output: revm::primitives::Bytes,
    },

    /// The execution of a transaction halted unexpectedly.
    #[error("execution failed to succeed due to halt!\n reason is: {reason:?}\n gas used is: {gas_used}")]
    ExecutionHalt {
        /// Provides the reason for the halt.
        reason: revm::primitives::Halt,

        /// Provides the amount of gas used by the transaction.
        gas_used: u64,
    },
}

impl MiddlewareError for RevmMiddlewareError {
    type Inner = Self;

    fn from_err(e: Self::Inner) -> Self {
        e
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        None
    }
}

impl RevmMiddleware {
    /// Creates a new instance of `RevmMiddleware` with procedurally generated
    /// signer/address if provided a seed/label and otherwise a random
    /// signer if not.
    ///
    /// # Examples
    /// ```
    /// use arbiter_core::{
    ///     environment::{BlockType, EnvironmentParameters},
    ///     manager::Manager,
    ///     middleware::RevmMiddleware,
    /// };
    ///
    /// let mut manager = Manager::new();
    /// let params = EnvironmentParameters {
    ///     label: "example_env".to_string(),
    ///     block_type: BlockType::RandomlySampled {
    ///         block_rate: 1.0,
    ///         block_time: 12,
    ///         seed: 1,
    ///     },
    /// };
    /// manager.add_environment(params).unwrap();
    /// let environment = manager.environments.get("example_env").unwrap();
    /// let middleware = RevmMiddleware::new(&environment, Some("test_label".to_string()));
    ///
    /// // We can create a middleware instance without a seed by doing the following
    /// let no_seed_middleware = RevmMiddleware::new(&environment, None);
    /// ```
    /// Use a seed if you want to have a constant address across simulations as
    /// well as a label for a client. This can be useful for debugging.
    pub fn new(environment: &Environment, seed_and_label: Option<String>) -> Self {
        let instruction_sender = environment.socket.instruction_sender.clone();
        let (outcome_sender, outcome_receiver) = crossbeam_channel::unbounded();
        let connection = Connection {
            instruction_sender,
            outcome_sender,
            outcome_receiver,
            event_broadcaster: Arc::clone(&environment.socket.event_broadcaster),
            filter_receivers: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            environment_state: Arc::clone(&environment.state),
        };
        let provider = Provider::new(connection);
        if let Some(seed) = seed_and_label {
            let mut hasher = Sha256::new();
            hasher.update(seed.clone());
            let hashed = hasher.finalize();
            let mut rng: StdRng = SeedableRng::from_seed(hashed.into());
            let wallet = Wallet::new(&mut rng);
            Self { provider, wallet }
        } else {
            let mut rng = rand::thread_rng();
            let wallet = Wallet::new(&mut rng);
            Self { provider, wallet }
        }
    }

    /// Allows the user to update the block number and timestamp of the
    /// [`Environment`] to whatever they may choose at any time.
    /// This can only be done when the [`Environment`] has
    /// [`EnvironmentParameters`] `block_type` field set to
    /// [`BlockType::UserControlled`].
    pub fn update_block(
        &self,
        block_number: impl Into<ethers::types::U256>,
        block_timestamp: impl Into<ethers::types::U256>,
    ) -> Result<ReceiptData, RevmMiddlewareError> {
        let block_number: ethers::types::U256 = block_number.into();
        let block_timestamp: ethers::types::U256 = block_timestamp.into();
        let provider = self.provider.as_ref();
        provider
            .instruction_sender
            .send(Instruction::BlockUpdate {
                block_number: block_number.into(),
                block_timestamp: block_timestamp.into(),
                outcome_sender: provider.outcome_sender.clone(),
            })
            .map_err(|e| RevmMiddlewareError::Send(e.to_string()))?;
        match provider.outcome_receiver.recv() {
            Ok(Ok(Outcome::BlockUpdateCompleted(receipt_data))) => Ok(receipt_data),
            _ => Err(RevmMiddlewareError::MissingData(
                "Block did not update Succesfully".to_string(),
            )),
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RevmMiddleware {
    type Provider = Connection;
    type Error = RevmMiddlewareError;
    type Inner = Self;

    /// Returns a reference to the inner middleware of which there is none when
    /// using [`RevmMiddleware`] so we relink to `Self`
    fn inner(&self) -> &Self::Inner {
        self
    }

    /// Provides access to the associated Ethereum provider which is given by
    /// the [`Provider<Connection>`] for [`RevmMiddleware`].
    fn provider(&self) -> &Provider<Self::Provider> {
        &self.provider
    }

    /// Provides the default sender address for transactions, i.e., the address
    /// of the wallet/signer given to a client of the [`Environment`].
    fn default_sender(&self) -> Option<Address> {
        Some(self.wallet.address())
    }

    /// Sends a transaction to the [`Environment`] which acts as a simulated
    /// Ethereum network.
    ///
    /// The method checks if the transaction is either a call to an existing
    /// contract or a deploy of a new one, and constructs the necessary
    /// transaction environment used for `revm`-based transactions.
    /// It then sends this transaction for execution and returns the
    /// corresponding pending transaction.
    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        tx: T,
        _block: Option<BlockId>,
    ) -> Result<PendingTransaction<'_, Self::Provider>, Self::Error> {
        if self
            .provider()
            .as_ref()
            .environment_state
            .load(std::sync::atomic::Ordering::SeqCst)
            == crate::environment::State::Paused
        {
            return Err(RevmMiddlewareError::Send("Environment Paused".to_string()));
        }

        let tx: TypedTransaction = tx.into();

        // Check the `to` field of the transaction to determine if it is a call or a
        // deploy. If there is no `to` field, then it is a `Deploy` else it is a
        // `Call`.
        let transact_to = match tx.to_addr() {
            Some(to) => TransactTo::Call(B160::from(*to)),
            None => TransactTo::Create(CreateScheme::Create),
        };

        let tx_env = TxEnv {
            caller: B160::from(self.wallet.address()),
            gas_limit: u64::MAX,
            gas_price: U256::ZERO,
            gas_priority_fee: None,
            transact_to,
            value: U256::ZERO,
            data: bytes::Bytes::from(
                tx.data()
                    .ok_or(RevmMiddlewareError::MissingData(
                        "Data missing in transaction!".to_string(),
                    ))?
                    .to_vec(),
            ),
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        };
        let instruction = Instruction::Transaction {
            tx_env: tx_env.clone(),
            outcome_sender: self.provider.as_ref().outcome_sender.clone(),
        };
        self.provider()
            .as_ref()
            .instruction_sender
            .send(instruction)
            .map_err(|e| RevmMiddlewareError::Send(e.to_string()))?;

        let outcome = self.provider().as_ref().outcome_receiver.recv()??;

        if let Outcome::TransactionCompleted(execution_result, receipt_data) = outcome {
            let Success {
                _reason: _,
                _gas_used: gas_used,
                _gas_refunded: _,
                logs,
                output,
            } = unpack_execution_result(execution_result)?;

            let to: Option<ethers::types::H160> = match tx_env.transact_to {
                TransactTo::Call(address) => Some(address.into()),
                TransactTo::Create(_) => None,
            };

            // Note that this is technically not the correct construction on the tx hash
            // but untill we increment the nonce correctly this will do
            let sender = self.wallet.address();
            let data = tx_env.clone().data;
            let mut hasher = Sha256::new();
            hasher.update(sender.as_bytes());
            hasher.update(data.as_ref());
            let hash = hasher.finalize();

            let mut block_hasher = Sha256::new();
            block_hasher.update(receipt_data.block_number.to_string().as_bytes());
            let block_hash = block_hasher.finalize();
            let block_hash = Some(ethers::types::H256::from_slice(&block_hash));

            match output {
                Output::Create(_, address) => {
                    let tx_receipt = TransactionReceipt {
                        block_hash,
                        block_number: Some(receipt_data.block_number),
                        contract_address: Some(recast_address(address.unwrap())),
                        logs: logs.clone(),
                        from: sender,
                        gas_used: Some(gas_used.into()),
                        effective_gas_price: Some(tx_env.clone().gas_price.into()),
                        transaction_hash: ethers::types::TxHash::from_slice(&hash),
                        to,
                        cumulative_gas_used: receipt_data.cumulative_gas_per_block.into(),
                        status: Some(1.into()),
                        root: None,
                        logs_bloom: {
                            let mut bloom = Bloom::default();
                            for log in &logs {
                                bloom.accrue(BloomInput::Raw(&log.address.0));
                                for topic in log.topics.iter() {
                                    bloom.accrue(BloomInput::Raw(topic.as_bytes()));
                                }
                            }
                            bloom
                        },
                        transaction_type: match tx {
                            TypedTransaction::Eip2930(_) => Some(1.into()),
                            _ => None,
                        },
                        transaction_index: receipt_data.transaction_index,
                        ..Default::default()
                    };

                    // TODO: I'm not sure we need to set the confirmations.
                    let mut pending_tx =
                        PendingTransaction::new(ethers::types::H256::zero(), self.provider())
                            .interval(Duration::ZERO)
                            .confirmations(0);

                    let state_ptr: *mut PendingTxState =
                        &mut pending_tx as *mut _ as *mut PendingTxState;

                    // Modify the value (this assumes you have access to the enum variants)
                    unsafe {
                        *state_ptr = PendingTxState::CheckingReceipt(Some(tx_receipt));
                    }

                    Ok(pending_tx)
                }
                Output::Call(_) => {
                    let tx_receipt = TransactionReceipt {
                        block_hash,
                        block_number: Some(receipt_data.block_number),
                        contract_address: None,
                        logs: logs.clone(),
                        from: sender,
                        gas_used: Some(gas_used.into()),
                        effective_gas_price: Some(tx_env.clone().gas_price.into()),
                        transaction_hash: ethers::types::TxHash::from_slice(&hash),
                        to,
                        cumulative_gas_used: receipt_data.cumulative_gas_per_block.into(),
                        status: Some(1.into()),
                        root: None,
                        logs_bloom: {
                            let mut bloom = Bloom::default();
                            for log in &logs {
                                bloom.accrue(BloomInput::Raw(&log.address.0));
                                for topic in log.topics.iter() {
                                    bloom.accrue(BloomInput::Raw(topic.as_bytes()));
                                }
                            }
                            bloom
                        },
                        transaction_type: match tx {
                            TypedTransaction::Eip2930(_) => Some(1.into()),
                            _ => None,
                        },
                        transaction_index: receipt_data.transaction_index,
                        ..Default::default()
                    };

                    // TODO: Create the actual tx_hash
                    // TODO: I'm not sure we need to set the confirmations.
                    let mut pending_tx =
                        PendingTransaction::new(ethers::types::H256::zero(), self.provider())
                            .interval(Duration::ZERO)
                            .confirmations(0);

                    let state_ptr: *mut PendingTxState =
                        &mut pending_tx as *mut _ as *mut PendingTxState;

                    // Modify the value (this assumes you have access to the enum variants)
                    unsafe {
                        *state_ptr = PendingTxState::CheckingReceipt(Some(tx_receipt));
                    }

                    Ok(pending_tx)
                }
            }
        } else {
            panic!("This should never happen!")
        }
    }

    /// Calls a contract method without creating a worldstate-changing
    /// transaction on the [`Environment`] (again, simulating the Ethereum
    /// network).
    ///
    /// Similar to `send_transaction`, this method checks if the call is
    /// targeting an existing contract or deploying a new one. After
    /// executing the call, it returns the output, but no worldstate change will
    /// be documented in the `revm` DB.
    async fn call(
        &self,
        tx: &TypedTransaction,
        _block: Option<BlockId>,
    ) -> Result<Bytes, Self::Error> {
        if self
            .provider()
            .as_ref()
            .environment_state
            .load(std::sync::atomic::Ordering::SeqCst)
            == crate::environment::State::Paused
        {
            return Err(RevmMiddlewareError::Send("Environment Paused".to_string()));
        }
        let tx = tx.clone();

        // Check the `to` field of the transaction to determine if it is a call or a
        // deploy. If there is no `to` field, then it is a `Deploy` else it is a
        // `Call`.
        let transact_to = match tx.to_addr() {
            Some(to) => TransactTo::Call(B160::from(*to)),
            None => TransactTo::Create(CreateScheme::Create),
        };
        let tx_env = TxEnv {
            caller: B160::from(self.wallet.address()),
            gas_limit: u64::MAX,
            gas_price: U256::ZERO,
            gas_priority_fee: None,
            transact_to,
            value: U256::ZERO,
            data: bytes::Bytes::from(
                tx.data()
                    .ok_or(RevmMiddlewareError::MissingData(
                        "Data missing in transaction!".to_string(),
                    ))?
                    .to_vec(),
            ),
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        };
        let instruction = Instruction::Call {
            tx_env,
            outcome_sender: self.provider().as_ref().outcome_sender.clone(),
        };
        self.provider()
            .as_ref()
            .instruction_sender
            .send(instruction)
            .map_err(|e| RevmMiddlewareError::Send(e.to_string()))?;
        let outcome = self.provider().as_ref().outcome_receiver.recv()??;

        if let Outcome::CallCompleted(execution_result) = outcome {
            let output = unpack_execution_result(execution_result)?.output;
            match output {
                Output::Create(bytes, ..) => {
                    return Ok(Bytes::from(bytes.to_vec()));
                }
                Output::Call(bytes) => {
                    return Ok(Bytes::from(bytes.to_vec()));
                }
            }
        } else {
            panic!("This should never happen!")
        }
    }

    /// Creates a new filter for incoming Ethereum logs based on certain
    /// criteria.
    ///
    /// Currently, this method supports log filters. Other filters like
    /// `NewBlocks` and `PendingTransactions` are not yet implemented.
    async fn new_filter(&self, filter: FilterKind<'_>) -> Result<ethers::types::U256, Self::Error> {
        let (_method, args) = match filter {
            FilterKind::NewBlocks => unimplemented!(
                "Filtering via new `FilterKind::NewBlocks` has not been implemented yet!"
            ),
            FilterKind::PendingTransactions => {
                unimplemented!("Filtering via `FilterKind::PendingTransactions` has not been implemented yet! 
                At the current development stage of Arbiter, transactions do not actually sit in a pending state
                 -- they are executed immediately.")
            }
            FilterKind::Logs(filter) => ("eth_newFilter", filter),
        };
        let filter = args.clone();
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&args).map_err(RevmMiddlewareError::Json)?);
        let hash = hasher.finalize();
        let id = ethers::types::U256::from(ethers::types::H256::from_slice(&hash).as_bytes());
        let (event_sender, event_receiver) =
            crossbeam_channel::unbounded::<Vec<revm::primitives::Log>>();
        let filter_receiver = FilterReceiver {
            filter,
            receiver: event_receiver,
        };
        self.provider()
            .as_ref()
            .event_broadcaster
            .lock()
            .map_err(|e| {
                RevmMiddlewareError::EventBroadcaster(format!(
                    "Failed to gain lock on the `Connection`'s `event_broadcaster` due to {:?} ",
                    e
                ))
            })?
            .add_sender(event_sender);
        self.provider()
            .as_ref()
            .filter_receivers
            .lock()
            .await
            .insert(id, filter_receiver);
        Ok(id)
    }

    /// Starts watching for logs that match a specific filter.
    ///
    /// This method creates a filter watcher that continuously checks for new
    /// logs matching the criteria in a separate thread.
    async fn watch<'b>(
        &'b self,
        filter: &Filter,
    ) -> Result<FilterWatcher<'b, Self::Provider, Log>, Self::Error> {
        let id = self.new_filter(FilterKind::Logs(filter)).await?;
        Ok(FilterWatcher::new(id, self.provider()).interval(Duration::ZERO))
    }
}

/// Represents a connection to the EVM contained in the corresponding
/// [`Environment`].
#[derive(Debug)]
pub struct Connection {
    /// Used to send calls and transactions to the [`Environment`] to be
    /// executed by `revm`.
    instruction_sender: InstructionSender,

    /// Used to send results back to a client that made a call/transaction with
    /// the [`Environment`]. This [`ResultSender`] is passed along with a
    /// call/transaction so the [`Environment`] can reply back with the
    /// [`ExecutionResult`].
    outcome_sender: OutcomeSender,

    /// Used to receive the [`ExecutionResult`] from the [`Environment`] upon
    /// call/transact.
    outcome_receiver: OutcomeReceiver,

    /// A reference to the [`EventBroadcaster`] so that more receivers of the
    /// broadcast can be taken from it.
    event_broadcaster: Arc<Mutex<EventBroadcaster>>,

    /// A collection of `FilterReceiver`s that will receive outgoing logs
    /// generated by `revm` and output by the [`Environment`].
    filter_receivers: Arc<tokio::sync::Mutex<HashMap<ethers::types::U256, FilterReceiver>>>,

    environment_state: Arc<crate::environment::AtomicState>,
}

#[async_trait::async_trait]
impl JsonRpcClient for Connection {
    type Error = ProviderError;

    /// Processes a JSON-RPC request and returns the response.
    /// Currently only handles the `eth_getFilterChanges` call since this is
    /// used for polling events emitted from the [`Environment`].
    async fn request<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ProviderError> {
        match method {
            "eth_getFilterChanges" => {
                // TODO: The extra json serialization/deserialization can probably be avoided
                // somehow

                // Get the `Filter` ID from the params `T`
                // First convert it into a JSON `Value`
                let value = serde_json::to_value(&params)?;

                // Take this value as an array then cast it to a string
                let str = value.as_array().ok_or(ProviderError::CustomError(
                    "The params value passed to the `Connection` via a `request` was empty. 
                    This is likely due to not specifying a specific `Filter` ID!".to_string()
                ))?[0]
                    .as_str().ok_or(ProviderError::CustomError(
                        "The params value passed to the `Connection` via a `request` could not be later cast to `str`!".to_string()
                    ))?;

                // Now get the `U256` ID via the string decoded from hex radix.
                let id = ethers::types::U256::from_str_radix(str, 16)
                    .map_err(|e| ProviderError::CustomError(
                        format!("The `str` representation of the filter ID could not be cast into `U256` due to: {:?}!", 
                        e)))?;

                // Get the corresponding `filter_receiver` and await for logs to appear.
                let mut filter_receivers = self.filter_receivers.lock().await;
                let filter_receiver =
                    filter_receivers
                        .get_mut(&id)
                        .ok_or(ProviderError::CustomError(
                            "The filter ID does not seem to match any that this client owns!"
                                .to_string(),
                        ))?;
                let mut logs = vec![];
                let filtered_params = FilteredParams::new(Some(filter_receiver.filter.clone()));
                if let Ok(received_logs) = filter_receiver.receiver.recv() {
                    let ethers_logs = revm_logs_to_ethers_logs(received_logs);
                    for log in ethers_logs {
                        if filtered_params.filter_address(&log)
                            && filtered_params.filter_topics(&log)
                        {
                            logs.push(log);
                        }
                    }
                }
                // Take the logs and Stringify then JSONify to cast into `R`.
                let logs_str = serde_json::to_string(&logs)?;
                let logs_deserializeowned: R = serde_json::from_str(&logs_str)?;
                return Ok(logs_deserializeowned);
            }
            var => {
                unimplemented!("We don't cover this case yet: {}", var);
            }
        }
    }
}

/// Packages together a [`crossbeam_channel::Receiver<Vec<Log>>`] along with a
/// [`Filter`] for events. Allows the client to have a stream of filtered
/// events.
#[derive(Debug)]
pub(crate) struct FilterReceiver {
    /// The filter definition used for this receiver.
    /// Comes from the `ethers-rs` crate.
    pub(crate) filter: Filter,

    /// The receiver for the channel that receives logs from the broadcaster.
    /// These are filtered upon reception.
    pub(crate) receiver: crossbeam_channel::Receiver<Vec<revm::primitives::Log>>,
}

/// Contains the result of a successful transaction execution.
#[derive(Debug)]
struct Success {
    _reason: revm::primitives::Eval,
    _gas_used: u64,
    _gas_refunded: u64,
    logs: Vec<ethers::types::Log>,
    output: Output,
}

/// Unpacks the result of the EVM execution.
///
/// This function converts the raw execution result from the EVM into a more
/// structured [`Success`] type or an error indicating the failure of the
/// execution.
fn unpack_execution_result(
    execution_result: ExecutionResult,
) -> Result<Success, RevmMiddlewareError> {
    match execution_result {
        ExecutionResult::Success {
            reason,
            gas_used,
            gas_refunded,
            logs,
            output,
        } => {
            let logs = revm_logs_to_ethers_logs(logs);
            Ok(Success {
                _reason: reason,
                _gas_used: gas_used,
                _gas_refunded: gas_refunded,
                logs,
                output,
            })
        }
        ExecutionResult::Revert { gas_used, output } => {
            Err(RevmMiddlewareError::ExecutionRevert { gas_used, output })
        }
        ExecutionResult::Halt { reason, gas_used } => {
            Err(RevmMiddlewareError::ExecutionHalt { reason, gas_used })
        }
    }
}

/// Converts logs from the Revm format to the Ethers format.
///
/// This function iterates over a list of logs as they appear in the `revm` and
/// converts each log entry to the corresponding format used by the `ethers-rs`
/// library.
#[inline]
fn revm_logs_to_ethers_logs(
    revm_logs: Vec<revm::primitives::Log>,
) -> Vec<ethers::core::types::Log> {
    let mut logs: Vec<ethers::core::types::Log> = vec![];
    for revm_log in revm_logs {
        let topics = revm_log.topics.into_iter().map(recast_b256).collect();
        let log = ethers::core::types::Log {
            address: recast_address(revm_log.address),
            topics,
            data: ethers::core::types::Bytes::from(revm_log.data),
            block_hash: None,
            block_number: None,
            transaction_hash: None,
            transaction_index: None,
            log_index: None,
            transaction_log_index: None,
            log_type: None,
            removed: None,
        };
        logs.push(log);
    }
    logs
}

// Certainly will go away with alloy-types
/// Recast a B160 into an Address type
/// # Arguments
/// * `address` - B160 to recast. (B160)
/// # Returns
/// * `Address` - Recasted Address.
#[inline]
pub fn recast_address(address: B160) -> Address {
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}

/// Recast a B256 into an H256 type
/// # Arguments
/// * `input` - B256 to recast. (B256)
/// # Returns
/// * `H256` - Recasted H256.
#[inline]
pub fn recast_b256(input: revm::primitives::B256) -> ethers::types::H256 {
    let temp: [u8; 32] = input.as_bytes().try_into().unwrap();
    ethers::types::H256::from(temp)
}

#[cfg(target_arch = "wasm32")]
pub(crate) type PinBoxFut<'a, T> = Pin<Box<dyn Future<Output = Result<T, ProviderError>> + 'a>>;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type PinBoxFut<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, ProviderError>> + Send + 'a>>;

// Because this is the exact same struct it will have the exact same memory
// aliment allowing us to bypass the fact that ethers-rs doesn't export this
// enum normally We box the TransactionReceipts to keep the enum small.
#[allow(unused, missing_docs)]
pub enum PendingTxState<'a> {
    /// Initial delay to ensure the GettingTx loop doesn't immediately fail
    InitialDelay(Pin<Box<Delay>>),

    /// Waiting for interval to elapse before calling API again
    PausedGettingTx,

    /// Polling The blockchain to see if the Tx has confirmed or dropped
    GettingTx(PinBoxFut<'a, Option<Transaction>>),

    /// Waiting for interval to elapse before calling API again
    PausedGettingReceipt,

    /// Polling the blockchain for the receipt
    GettingReceipt(PinBoxFut<'a, Option<TransactionReceipt>>),

    /// If the pending tx required only 1 conf, it will return early. Otherwise
    /// it will proceed to the next state which will poll the block number
    /// until there have been enough confirmations
    CheckingReceipt(Option<TransactionReceipt>),

    /// Waiting for interval to elapse before calling API again
    PausedGettingBlockNumber(Option<TransactionReceipt>),

    /// Polling the blockchain for the current block number
    GettingBlockNumber(PinBoxFut<'a, U64>, Option<TransactionReceipt>),

    /// Future has completed and should panic if polled again
    Completed,
}
