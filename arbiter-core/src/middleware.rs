#![warn(missing_docs)]
#![allow(clippy::all)]
//! This module contains the middleware for the Revm simulation environment.
//! Most of the middleware is essentially a placeholder, but it is necessary to have a middleware to work with bindings more efficiently.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{fmt::Debug, time::Duration};

use ethers::providers::JsonRpcClient;
use ethers::types::FilteredParams;
use ethers::{
    prelude::{
        k256::{
            ecdsa::SigningKey,
            sha2::{Digest, Sha256},
        },
        pending_transaction::PendingTxState,
        ProviderError,
    },
    providers::{FilterKind, FilterWatcher, Middleware, PendingTransaction, Provider},
    signers::{Signer, Wallet},
    types::{transaction::eip2718::TypedTransaction, Address, BlockId, Bytes, Filter, Log},
};
use rand::{rngs::StdRng, SeedableRng};
use revm::primitives::{
    CreateScheme, ExecutionResult, Output, TransactTo, TxEnv, B160, B256, U256,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::environment::{EventBroadcaster, ResultReceiver, ResultSender, TxSender};
use crate::{
    agent::{Agent, NotAttached},
    environment::Environment,
    utils::{recast_address, revm_logs_to_ethers_logs},
};

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

impl Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("tx_sender", &"TxSender")
            .field("result_sender", &"ResultSender")
            .field("result_receiver", &"ResultReceiver")
            .field(
                "filter_receivers",
                &"HashMap<ethers::types::U256, FilterReceiver>",
            )
            .finish()
    }
}

// TODO: This below seems needlessly clunky. There is a lot of serialize/deserialize and I bet we can avoid it and avoid a match all together.
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
                println!("id: {:?}", id);
                let mut filter_receivers = self.filter_receivers.lock().await;
                let filter_receiver = filter_receivers.get_mut(&id).unwrap();
                println!(
                    "filter_receiver in eth_getFilterChanges: {:?}",
                    filter_receiver
                );
                let mut logs = vec![];
                let filtered_params = FilteredParams::new(Some(filter_receiver.filter.clone()));
                while let Ok(received_logs) = filter_receiver.receiver.recv() {
                    println!("received_logs: {:?}", received_logs);
                    for log in received_logs {
                        println!("conditionals:\n address: {:?}\n topics: {:?}\n", filtered_params.filter_address(&log), filtered_params.filter_topics(&log));
                        if filtered_params.filter_address(&log) && filtered_params.filter_topics(&log)
                        {
                            logs.push(log);
                        }
                    }
                    break
                }
                // let logs = filter_receiver.receiver.recv().unwrap();
                println!("logs: {:?}", logs);
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
    pub fn new(agent: &Agent<NotAttached>, environment: &Environment) -> Self {
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
        hasher.update(agent.name.as_bytes());
        let seed = hasher.finalize();
        let mut rng = StdRng::from_seed(seed.into());
        let wallet = Wallet::new(&mut rng);
        Self { provider, wallet }
    }
}

#[async_trait::async_trait]
impl Middleware for RevmMiddleware {
    /// The JSON-RPC client type at the bottom of the stack
    type Provider = Connection;
    /// Error type returned by most operations
    type Error = ProviderError; //RevmMiddlewareError;
    /// The next-lower middleware in the middleware stack
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

    /// sending a transaction to revm is the same as committing a transaction and it won't return the output of the call but will cause logs to echo. Deploys are ran through here as well.
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

    /// Makes a call to revm that will not commit a state change to the DB. Can be used for a mock transaction
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
        // TODO: Modify this to work for calls/deploys
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
            ExecutionResult::Revert { output, .. } => panic!("Failed due to revert: {:?}", output),
            ExecutionResult::Halt { reason, .. } => panic!("Failed due to halt: {:?}", reason),
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

    // NOTES: It might be good to have individual channels for the EVM to send events to so that an agent can install a filter and the logs can be filtered by the EVM itself.
    // This could be handled similarly to how broadcasts are done now and maybe nothing there needs to change except for attaching a filter to the event channels.
    // It would be good to also pass to a separate thread to do broadcasting if we aren't already doing that so that the EVM can process while events are being sent out.
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
