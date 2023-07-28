#![warn(missing_docs)]
#![allow(clippy::all)]
//! This module contains the middleware for the Revm simulation environment.
//! Most of the middleware is essentially a placeholder, but it is necessary to have a middleware to work with bindings more efficiently.

use std::fmt::Debug;
use ethers::prelude::pending_transaction::PendingTxState;
use ethers::providers::{PendingTransaction, Provider, FilterKind};

use ethers::{
    prelude::{
        k256::{
            ecdsa::SigningKey,
            sha2::{Digest, Sha256},
        },
        pending_transaction::PendingTxState,
        ProviderError,
    },
    providers::{FilterWatcher, Middleware, MockProvider, PendingTransaction, Provider},
    signers::{Signer, Wallet},
    types::{transaction::eip2718::TypedTransaction, Address, BlockId, Bytes, Filter, Log},
};
use rand::{rngs::StdRng, SeedableRng};
use revm::primitives::{CreateScheme, ExecutionResult, Output, TransactTo, TxEnv, B160, U256};

use std::fmt::Debug;
use std::time::Duration;

use crate::{
    environment::Connection,
    utils::{recast_address, revm_logs_to_ethers_logs},
};

// TODO: Refactor the connection and channels slightly to be more intuitive. For instance, the middleware may not really need to own a connection, but input one to set up everything else?
#[derive(Debug)]
pub struct RevmMiddleware {
    provider: Provider<MockProvider>,
    connection: Connection,
    wallet: Wallet<SigningKey>,
    result_sender: crossbeam_channel::Sender<ExecutionResult>,
    result_receiver: crossbeam_channel::Receiver<ExecutionResult>,
    event_receiver: crossbeam_channel::Receiver<Vec<Log>>,
}

impl RevmMiddleware {
    pub fn new(connection: Connection, name: String) -> Self {
        let provider = Provider::new(MockProvider::new());
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let seed = hasher.finalize();
        let mut rng = StdRng::from_seed(seed.into());
        let wallet = Wallet::new(&mut rng);
        let (result_sender, result_receiver) = crossbeam_channel::unbounded();
        let (event_sender, event_receiver) = crossbeam_channel::unbounded();
        connection
            .event_broadcaster
            .lock()
            .unwrap()
            .add_sender(event_sender);
        Self {
            provider,
            connection,
            wallet,
            result_sender,
            result_receiver,
            event_receiver,
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RevmMiddleware {
    /// The JSON-RPC client type at the bottom of the stack
    type Provider = MockProvider;
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
        block: Option<BlockId>,
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
        self.connection
            .tx_sender
            .send((true, tx_env.clone(), self.result_sender.clone()))
            .unwrap();
        let result = self.result_receiver.recv().unwrap();
        let (output, revm_logs) = match result.clone() {
            ExecutionResult::Success { output, logs, .. } => (output, logs),
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

                pending_tx.state = PendingTxState::RevmTransactOutput(logs);
                return Ok(pending_tx);
            }
        }

        // TODO: RECEIPTS OF TRANSACTIONS SHOULD STORE THE BLOCKNUMBERS THEY OCCURED IN
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
        self.connection
            .tx_sender
            .send((false, tx_env.clone(), self.result_sender.clone()))
            .unwrap();
        let result = self.result_receiver.recv().unwrap();
        let output = match result.clone() {
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
    async fn new_filter(&self, filter: FilterKind<'_>) -> Result<ethers::types::U256, ProviderError> {
        todo!()
        // let (method, args) = match filter {
        //     FilterKind::NewBlocks => unimplemented!("We will need to implement this."),
        //     FilterKind::PendingTransactions => unimplemented!("Not sure if we need to implement this."),
        //     FilterKind::Logs(filter) => ("eth_newFilter", vec![utils::serialize(&filter)]),
        // };

        // self.request(method, args).await
    }

    async fn watch<'a>(
        &'a self,
        filter: &Filter,
    ) -> Result<FilterWatcher<'a, Self::Provider, Log>, Self::Error> {
        let id = self.new_filter(FilterKind::Logs(filter)).await?;
        Ok(FilterWatcher::new(id, self.provider()).interval(Duration::ZERO))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    async fn send_transaction() {
        todo!("Send shouldn't be called as we just need to FILL transactions.")
    }

    async fn call() {
        todo!("we should be able to call. We will have to consider adding a function to the `SimulationEnvironment` that uses `transact` and not `transact_commit`")
    }

    async fn get_logs() {
        todo!("we should be able to get logs.")
    }

    async fn watch() {
        todo!("we should be able to watch. we already have this partially implemented for agents.")
    }
}
