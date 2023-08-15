#![warn(missing_docs, unsafe_code)]

// TODO: Check the publicness of all structs and functions.

use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
    time::Duration,
};

use ethers::{
    core::rand::{thread_rng, SeedableRng},
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

use crate::{
    // agent::{Agent, NotAttached},
    environment::{Environment, EventBroadcaster, ResultReceiver, ResultSender, TxSender},
};

#[derive(Debug)]
pub struct RevmMiddleware {
    provider: Provider<Connection>,
    wallet: Wallet<SigningKey>,
}

#[derive(Error, Debug)]
pub enum RevmMiddlewareError {
    #[error("failed to send transaction! due to: {cause}")]
    SendError { cause: String },

    #[error("missing data! due to: {cause}")]
    MissingDataError { cause: String },

    #[error("failed to convert types! due to: {cause}")]
    ConversionError { cause: String },

    #[error("failed to receive `ExecutionResult`! due to: {cause}")]
    ReceiveError { cause: String },

    #[error("execution failed to succeed due to revert! output is: {cause}")]
    ExecutionRevert { cause: String },

    #[error("execution failed to succeed due to halt! output is: {cause}")]
    ExecutionHalt { cause: String },

    #[error("failed to handle with JSON data! due to: {cause}")]
    JsonError { cause: serde_json::Error },

    #[error("failed to gain event broadcaster lock! due to: {cause}")]
    EventBroadcasterError { cause: String },
}

impl MiddlewareError for RevmMiddlewareError {
    type Inner = Self;

    fn from_err(e: Self::Inner) -> Self {
        e
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        Some(self)
    }
}

impl RevmMiddleware {
    pub fn new(environment: &Environment, seed_and_label: Option<String>) -> Self {
        let tx_sender = environment.socket.tx_sender.clone();
        let (result_sender, result_receiver) = crossbeam_channel::unbounded();
        let connection = Connection {
            tx_sender,
            result_sender,
            result_receiver,
            event_broadcaster: Arc::clone(&environment.socket.event_broadcaster),
            filter_receivers: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
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
            let mut rng = thread_rng();
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

    fn inner(&self) -> &Self::Inner {
        &self
    }

    fn provider(&self) -> &Provider<Self::Provider> {
        &self.provider
    }

    fn default_sender(&self) -> Option<Address> {
        Some(self.wallet.address())
    }

    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        tx: T,
        _block: Option<BlockId>,
    ) -> Result<PendingTransaction<'_, Self::Provider>, Self::Error> {
        let tx: TypedTransaction = tx.into();

        // Check the `to` field of the transaction to determine if it is a call or a deploy.
        // If there is no `to` field, then it is a `Deploy` else it is a `Call`.
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
                    .ok_or(RevmMiddlewareError::MissingDataError {
                        cause: "Data missing in transaction!".to_string(),
                    })?
                    .to_vec(),
            ),
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        };
        println!("gotten past creating txenv");
        self.provider()
            .as_ref()
            .tx_sender
            .send((
                true,
                tx_env.clone(),
                self.provider().as_ref().result_sender.clone(),
            ))
            .map_err(|e| RevmMiddlewareError::SendError {
                cause: e.to_string(),
            })?;
        println!("sent to provider");
        let revm_result = self
            .provider()
            .as_ref()
            .result_receiver
            .recv()
            .map_err(|e| RevmMiddlewareError::ReceiveError {
                cause: e.to_string(),
            })?;

        let Success {
            _reason: _,
            _gas_used: _,
            _gas_refunded: _,
            logs,
            output,
        } = unpack_execution_result(revm_result.result)?;
        match output {
            Output::Create(_, address) => {
                let address = address.ok_or(RevmMiddlewareError::MissingDataError {
                    cause: "Address missing in transaction!".to_string(),
                })?;
                let mut pending_tx =
                    PendingTransaction::new(ethers::types::H256::zero(), self.provider());
                pending_tx.state = PendingTxState::RevmDeployOutput(recast_address(address));
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

    async fn call(
        &self,
        tx: &TypedTransaction,
        _block: Option<BlockId>,
    ) -> Result<Bytes, Self::Error> {
        let tx = tx.clone();

        // Check the `to` field of the transaction to determine if it is a call or a deploy.
        // If there is no `to` field, then it is a `Deploy` else it is a `Call`.
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
                    .ok_or(RevmMiddlewareError::MissingDataError {
                        cause: "Data missing in transaction!".to_string(),
                    })?
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
            .map_err(|e| RevmMiddlewareError::SendError {
                cause: e.to_string(),
            })?;
        let revm_result = self
            .provider()
            .as_ref()
            .result_receiver
            .recv()
            .map_err(|e| RevmMiddlewareError::ReceiveError {
                cause: e.to_string(),
            })?;
        let output = unpack_execution_result(revm_result.result)?.output;
        match output {
            Output::Create(bytes, ..) => {
                return Ok(Bytes::from(bytes.to_vec()));
            }
            Output::Call(bytes) => {
                return Ok(Bytes::from(bytes.to_vec()));
            }
        }
    }

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
        println!("args: {:?}", args);
        let filter = args.clone();
        let mut hasher = Sha256::new();
        hasher.update(
            serde_json::to_string(&args)
                .map_err(|e| RevmMiddlewareError::JsonError { cause: e })?,
        );
        let hash = hasher.finalize();
        let id = ethers::types::U256::from(ethers::types::H256::from_slice(&hash).as_bytes());
        let (event_sender, event_receiver) = crossbeam_channel::unbounded();
        let filter_receiver = FilterReceiver {
            filter,
            receiver: event_receiver,
        };
        self.provider()
            .as_ref()
            .event_broadcaster
            .lock()
            .map_err(|e| RevmMiddlewareError::EventBroadcasterError {
                cause: format!(
                    "Failed to gain lock on the `Connection`'s `event_broadcaster` due to {:?} ",
                    e
                ),
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

    async fn watch<'b>(
        &'b self,
        filter: &Filter,
    ) -> Result<FilterWatcher<'b, Self::Provider, Log>, Self::Error> {
        let id = self.new_filter(FilterKind::Logs(filter)).await?;
        Ok(FilterWatcher::new(id, self.provider()).interval(Duration::ZERO))
    }
}

#[derive(Debug)]
pub struct Connection {
    pub(crate) tx_sender: TxSender,
    pub(crate) result_sender: ResultSender,
    pub(crate) result_receiver: ResultReceiver,
    event_broadcaster: Arc<Mutex<EventBroadcaster>>,
    filter_receivers: Arc<tokio::sync::Mutex<HashMap<ethers::types::U256, FilterReceiver>>>,
}

#[async_trait::async_trait]
impl JsonRpcClient for Connection {
    type Error = ProviderError;

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
                while let Ok(received_logs) = filter_receiver.receiver.recv() {
                    for log in received_logs {
                        if filtered_params.filter_address(&log)
                            && filtered_params.filter_topics(&log)
                        {
                            logs.push(log);
                        }
                    }
                    break;
                }

                // TODO: This can probably be avoided somehow
                // Take the logs and Stringify then JSONify to cast into `R`.
                let logs_str = serde_json::to_string(&logs)?;
                let logs_deserializeowned: R = serde_json::from_str(&logs_str)?;
                return Ok(logs_deserializeowned);
            }
            _ => {
                unimplemented!("We don't cover this case yet.")
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct FilterReceiver {
    pub(crate) filter: Filter,
    pub(crate) receiver: crossbeam_channel::Receiver<Vec<ethers::types::Log>>,
}

struct Success {
    _reason: revm::primitives::Eval,
    _gas_used: u64,
    _gas_refunded: u64,
    logs: Vec<ethers::types::Log>,
    output: Output,
}

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
        ExecutionResult::Revert { gas_used, output } => Err(RevmMiddlewareError::ExecutionRevert {
            cause: format!(
                "Transaction reverted and used {} gas and output {:?}",
                gas_used, output
            ),
        }),
        ExecutionResult::Halt { reason, gas_used } => Err(RevmMiddlewareError::ExecutionHalt {
            cause: format!(
                "Transaction halted for reasond {:?} gas and output {:?}",
                reason, gas_used
            ),
        }),
    }
}

// TODO: These below could possibly be replaced by the relevant From<> or Into<> trait impls but we run into the orphan rule.
// This may not be worthwhile given the release of Alloy

// Certainly will go away with alloy-types
/// Recast a B160 into an Address type
/// # Arguments
/// * `address` - B160 to recast. (B160)
/// # Returns
/// * `Address` - Recasted Address.
#[inline]
fn recast_address(address: B160) -> Address {
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap(); // This unwrap should never fail as the `B160` will always cast into `[u8; 20]`.
    Address::from(temp)
}

/// Recast a B256 into an H256 type
/// # Arguments
/// * `input` - B256 to recast. (B256)  
/// # Returns
/// * `H256` - Recasted H256.
#[inline]
fn recast_b256(input: revm::primitives::B256) -> ethers::types::H256 {
    let temp: [u8; 32] = input.as_bytes().try_into().unwrap(); // This unwrap should never fail as the `B256` will always cast into `[u8; 32]`.
    ethers::types::H256::from(temp)
}

/// Recast a logs from Revm into the ethers.rs Log type.
/// # Arguments
/// * `revm_logs` - Logs from Revm. (Vec<revm::primitives::Log>)
/// # Returns
/// * `Vec<ethers::core::types::Log>` - Logs recasted into ethers.rs Log type.
#[inline]
pub fn revm_logs_to_ethers_logs(
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
