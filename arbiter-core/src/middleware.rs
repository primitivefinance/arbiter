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

#![warn(missing_docs, unsafe_code)]

// TODO: Check the publicness of all structs and functions.

use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
    time::Duration,
};

use ethers::{
    core::rand::SeedableRng,
    prelude::{
        k256::{
            ecdsa::SigningKey,
            sha2::{Digest, Sha256},
        },
        pending_transaction::PendingTxState,
        ProviderError,
    },
    providers::{
        FilterKind, FilterWatcher, JsonRpcClient, Middleware, MiddlewareError, PendingTransaction,
        Provider,
    },
    signers::{Signer, Wallet},
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockId, Bytes, Filter, FilteredParams,
        Log,
    },
};
use rand::rngs;
use revm::primitives::{CreateScheme, ExecutionResult, Output, TransactTo, TxEnv, B160, U256};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::environment::{
    Environment, EventBroadcaster, ResultReceiver, ResultSender, TransactionOutcome, TxSender,
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
/// use arbiter_core::{manager::Manager, middleware::RevmMiddleware};
///
/// // Create a manager and add an environment
/// let mut manager = Manager::new();
/// manager.add_environment("example_env", 1.0, 42).unwrap();
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
    /// An error occurred while attempting to send a transaction.
    #[error("failed to send transaction! due to: {0}")]
    Send(String),

    /// There was an issue receiving an [`ExecutionResult`], possibly from
    /// another service or module.
    #[error("failed to receive `ExecutionResult`! due to: {0}")]
    Receive(String),

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
    /// use arbiter_core::{manager::Manager, middleware::RevmMiddleware};
    ///
    /// let mut manager = Manager::new();
    /// manager.add_environment("example_env", 1.0, 42).unwrap();
    /// let environment = manager.environments.get("example_env").unwrap();
    /// let middleware = RevmMiddleware::new(&environment, Some("test_label".to_string()));
    ///
    /// // We can create a middleware instance without a seed by doing the following
    /// let no_seed_middleware = RevmMiddleware::new(&environment, None);
    /// ```
    /// Use a seed if you want to have a constant address across simulations as
    /// well as a label for a client. This can be useful for debugging.
    pub fn new(environment: &Environment, seed_and_label: Option<String>) -> Self {
        let tx_sender = environment.socket.tx_sender.clone();
        let (result_sender, result_receiver) = crossbeam_channel::unbounded();
        let connection = Connection {
            tx_sender,
            result_sender,
            result_receiver,
            event_broadcaster: Arc::clone(&environment.socket.event_broadcaster),
            filter_receivers: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            environment_state: Arc::clone(&environment.state),
        };
        let provider = Provider::new(connection);
        if let Some(seed) = seed_and_label {
            let mut hasher = Sha256::new();
            hasher.update(seed.clone());
            let hashed = hasher.finalize();
            let mut rng: rngs::StdRng = SeedableRng::from_seed(hashed.into());
            let wallet = Wallet::new(&mut rng);
            Self { provider, wallet }
        } else {
            let mut rng = rand::thread_rng();
            let wallet = Wallet::new(&mut rng);
            Self { provider, wallet }
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
        self.provider()
            .as_ref()
            .tx_sender
            .send((
                true,
                tx_env.clone(),
                self.provider().as_ref().result_sender.clone(),
            ))
            .map_err(|e| RevmMiddlewareError::Send(e.to_string()))?;

        let revm_result = self
            .provider()
            .as_ref()
            .result_receiver
            .recv()
            .map_err(|e| RevmMiddlewareError::Receive(e.to_string()))?;

        match revm_result.outcome {
            TransactionOutcome::Success(execution_result) => {
                let Success {
                    _reason: _,
                    _gas_used: _,
                    _gas_refunded: _,
                    logs,
                    output,
                } = unpack_execution_result(execution_result)?;

                match output {
                    Output::Create(_, address) => {
                        let address = address.ok_or(RevmMiddlewareError::MissingData(
                            "Address missing in transaction!".to_string(),
                        ))?;
                        let mut pending_tx =
                            PendingTransaction::new(ethers::types::H256::zero(), self.provider());
                        pending_tx.state =
                            PendingTxState::RevmDeployOutput(recast_address(address));
                        return Ok(pending_tx);
                    }
                    Output::Call(_) => {
                        let mut pending_tx =
                            PendingTransaction::new(ethers::types::H256::zero(), self.provider());

                        pending_tx.state =
                            PendingTxState::RevmTransactOutput(logs, revm_result.block_number);
                        return Ok(pending_tx);
                    }
                }
            }
            TransactionOutcome::Error(err) => {
                return Err(RevmMiddlewareError::Receive(
                    format!(
                        "Error recieving response from the environement with environment error: {}",
                        err
                    )
                    .to_string(),
                ));
            }
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
        self.provider()
            .as_ref()
            .tx_sender
            .send((
                false,
                tx_env.clone(),
                self.provider().as_ref().result_sender.clone(),
            ))
            .map_err(|e| RevmMiddlewareError::Send(e.to_string()))?;
        let revm_result = self
            .provider()
            .as_ref()
            .result_receiver
            .recv()
            .map_err(|e| RevmMiddlewareError::Receive(e.to_string()))?;

        match revm_result.outcome {
            TransactionOutcome::Success(execution_result) => {
                let output = unpack_execution_result(execution_result)?.output;
                match output {
                    Output::Create(bytes, ..) => {
                        return Ok(Bytes::from(bytes.to_vec()));
                    }
                    Output::Call(bytes) => {
                        return Ok(Bytes::from(bytes.to_vec()));
                    }
                }
            }
            TransactionOutcome::Error(err) => {
                return Err(RevmMiddlewareError::Receive(
                    format!(
                        "Error recieving response from the environement with environment error: {}",
                        err
                    )
                    .to_string(),
                ));
            }
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
        hasher.update(serde_json::to_string(&args).map_err(|e| RevmMiddlewareError::Json(e))?);
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
    tx_sender: TxSender,

    /// Used to send results back to a client that made a call/transaction with
    /// the [`Environment`]. This [`ResultSender`] is passed along with a
    /// call/transaction so the [`Environment`] can reply back with the
    /// [`ExecutionResult`].
    result_sender: ResultSender,

    /// Used to receive the [`ExecutionResult`] from the [`Environment`] upon
    /// call/transact.
    result_receiver: ResultReceiver,

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
                // Get the `Filter` ID from the params `T`
                // First convert it into a JSON `Value`
                let value = serde_json::to_value(&params)?;

                // Take this value as an array then cast it to a string
                let str = value.as_array().ok_or(ProviderError::CustomError(format!(
                    "The params value passed to the `Connection` via a `request` was empty. 
                    This is likely due to not specifying a specific `Filter` ID!"
                )))?[0]
                    .as_str().ok_or(ProviderError::CustomError(format!(
                        "The params value passed to the `Connection` via a `request` could not be later cast to `str`!"
                    )))?;

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
                        .ok_or(ProviderError::CustomError(format!(
                            "The filter ID does not seem to match any that this client owns!"
                        )))?;
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
            _ => {
                unimplemented!("We don't cover this case yet.")
            } // TODO: This can probably be avoided somehow
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

/// Converts the address type used by `revm` to the one used by `ethers-rs`.
///
/// This inline function performs a straightforward transformation of the
/// address types. The provided address type from Revm is transformed into the
/// corresponding type used in the Ethers library.
#[inline]
fn recast_address(address: B160) -> Address {
    // This unwrap should never fail as the `B160` will always cast into `[u8; 20]`.
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}

/// Converts the 256-bit byte array type used by `revm` to the one used by
/// `ethers-rs`.
///
/// This inline function performs a simple transformation of the 256-bit byte
/// arrays. The provided byte array from Revm is transformed into the
/// corresponding type used in the Ethers library.
#[inline]
fn recast_b256(input: revm::primitives::B256) -> ethers::types::H256 {
    // This unwrap should never fail as the `B256` will always cast into `[u8; 32]`.
    let temp: [u8; 32] = input.as_bytes().try_into().unwrap();
    ethers::types::H256::from(temp)
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
