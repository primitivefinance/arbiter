#![warn(missing_docs, unsafe_code)]

// TODO: Check the publicness of all structs and functions.

use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
    time::Duration,
};

use ethers::{
    prelude::{
        k256::{
            ecdsa::SigningKey,
            sha2::{Digest, Sha256},
        },
        pending_transaction::PendingTxState,
        ProviderError,
    },
    providers::{
        FilterKind, FilterWatcher, JsonRpcClient, Middleware, PendingTransaction, Provider,
    },
    signers::{Signer, Wallet},
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockId, Bytes, Filter, FilteredParams,
        Log,
    },
};
use rand::{rngs::{StdRng, OsRng}, SeedableRng};
use revm::primitives::{CreateScheme, ExecutionResult, Output, TransactTo, TxEnv, B160, U256};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    // agent::{Agent, NotAttached},
    environment::{Environment, EventBroadcaster, ResultReceiver, ResultSender, TxSender},
};

#[derive(Debug)]
pub struct Connection {
    pub(crate) tx_sender: TxSender,
    pub(crate) result_sender: ResultSender,
    pub(crate) result_receiver: ResultReceiver,
    event_broadcaster: Arc<Mutex<EventBroadcaster>>,
    filter_receivers: Arc<tokio::sync::Mutex<HashMap<ethers::types::U256, FilterReceiver>>>,
}

#[derive(Debug)]
pub(crate) struct FilterReceiver {
    pub(crate) filter: Filter,
    pub(crate) receiver: crossbeam_channel::Receiver<Vec<ethers::types::Log>>,
}

#[allow(clippy::never_loop)]
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
                let value = serde_json::to_value(&params).unwrap();
                let str = value.as_array().unwrap()[0].as_str().unwrap();
                let id = ethers::types::U256::from_str_radix(str, 16).unwrap();
                let mut filter_receivers = self.filter_receivers.lock().await;
                let filter_receiver = filter_receivers.get_mut(&id).unwrap();
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
                let logs_str = serde_json::to_string(&logs).unwrap();
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
pub struct RevmMiddleware {
    provider: Provider<Connection>,
    wallet: Wallet<SigningKey>,
}

impl RevmMiddleware {
    pub fn new(environment: &Environment) -> Self {
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
        let mut hasher = Sha256::new();
        // hasher.update(agent.name.as_bytes());
        // let seed = hasher.finalize();
        let mut rng = OsRng::default();
        let wallet = Wallet::new(&mut rng);
        Self { provider, wallet }
    }
}

#[async_trait::async_trait]
impl Middleware for RevmMiddleware {
    type Provider = Connection;
    type Error = ProviderError;
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
        let mut tx: TypedTransaction = tx.into();
        tx.set_from(self.wallet.address());

        // Check the `to` field of the transaction to determine if it is a call or a deploy.
        // If there is no `to` field, then it is a `Deploy` else it is a `Call`.
        let transact_to = match tx.to_addr() {
            Some(to) => TransactTo::Call(B160::from(*to)),
            None => TransactTo::Create(CreateScheme::Create),
        };
        let tx_env = TxEnv {
            caller: B160::from(*tx.from().unwrap()),
            gas_limit: u64::MAX,
            gas_price: U256::ZERO,
            gas_priority_fee: None,
            transact_to,
            value: U256::ZERO,
            data: bytes::Bytes::from(tx.data().unwrap().clone().to_vec()),
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
            .unwrap();

        let revm_result = self.provider().as_ref().result_receiver.recv().unwrap();

        let (output, revm_logs, block) = match revm_result.result {
            ExecutionResult::Success { output, logs, .. } => {
                (output, logs, revm_result.block_number)
            }
            ExecutionResult::Revert { output, .. } => panic!("Failed due to revert: {:?}", output),
            ExecutionResult::Halt { reason, .. } => panic!("Failed due to halt: {:?}", reason),
        };
        match output {
            Output::Create(_, address) => {
                let mut pending_tx =
                    PendingTransaction::new(ethers::types::H256::zero(), self.provider());
                pending_tx.state =
                    PendingTxState::RevmDeployOutput(recast_address(address.unwrap()));
                return Ok(pending_tx);
            }
            Output::Call(_) => {
                let mut pending_tx =
                    PendingTransaction::new(ethers::types::H256::zero(), self.provider());
                let logs = revm_logs_to_ethers_logs(revm_logs);

                pending_tx.state = PendingTxState::RevmTransactOutput(logs, block);
                return Ok(pending_tx);
            }
        }
    }

    async fn call(
        &self,
        tx: &TypedTransaction,
        _block: Option<BlockId>,
    ) -> Result<Bytes, Self::Error> {
        let mut tx = tx.clone();
        tx.set_from(self.wallet.address());

        // Check the `to` field of the transaction to determine if it is a call or a deploy.
        // If there is no `to` field, then it is a `Deploy` else it is a `Call`.
        let transact_to = match tx.to_addr() {
            Some(to) => TransactTo::Call(B160::from(*to)),
            None => TransactTo::Create(CreateScheme::Create),
        };
        let tx_env = TxEnv {
            caller: B160::from(*tx.from().unwrap()),
            gas_limit: u64::MAX,
            gas_price: U256::ZERO,
            gas_priority_fee: None,
            transact_to,
            value: U256::ZERO,
            data: bytes::Bytes::from(tx.data().unwrap().clone().to_vec()),
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
            .unwrap();
        let revm_result = self.provider().as_ref().result_receiver.recv().unwrap();
        let output = match revm_result.result.clone() {
            ExecutionResult::Success { output, .. } => output,
            ExecutionResult::Revert { output, .. } => panic!("Failed due to revert: {:?}", output), /* TODO: We can throw an error here instead and pause the environment if need be. */
            ExecutionResult::Halt { reason, .. } => panic!("Failed due to halt: {:?}", reason), /* TODO: We can throw an error here instead and pause the environment if need be. */
        };
        match output {
            Output::Create(bytes, ..) => {
                return Ok(Bytes::from(bytes.to_vec()));
            }
            Output::Call(bytes) => {
                return Ok(Bytes::from(bytes.to_vec()));
            }
        }
    }

    async fn get_logs(&self, _filter: &Filter) -> Result<Vec<Log>, Self::Error> {
        todo!("we should be able to get logs.")
    }

    async fn new_filter(
        &self,
        filter: FilterKind<'_>,
    ) -> Result<ethers::types::U256, ProviderError> {
        let (_method, args) = match filter {
            FilterKind::NewBlocks => unimplemented!("We will want to implement this."),
            FilterKind::PendingTransactions => {
                unimplemented!("Not sure if we need to implement this.")
            }
            FilterKind::Logs(filter) => ("eth_newFilter", filter),
        };
        println!("args: {:?}", args);
        let filter = args.clone();
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&args).unwrap());
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
            .unwrap()
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
