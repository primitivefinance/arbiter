#![warn(missing_docs)]
#![allow(clippy::all)]
//! This module contains the middleware for the Revm simulation environment.
//! Most of the middleware is essentially a placeholder, but it is necessary to have a middleware to work with bindings more efficiently.

// TODO and notes:
// We should be able to just implement middleware for the Simulation/RevmEnvironment. 
// If we do this, we can have the environment own Agents and also each agent will own a wallet.
// We will have to see how we can let an agent make a call nicely, i.e., do something like
// ```
// writer = Writer::new(client, ());
// writer.echo_string("hello world");
// ```
// Each contract instance like this will be associated to the sender and this environment must be able to be referenced by the agent.


use ethers::{
    prelude::k256::ecdsa::SigningKey,
    prelude::ProviderError,
    providers::{
        FilterKind, FilterWatcher, LogQuery, Middleware, MiddlewareError, PendingTransaction,
        Provider,
    },
    signers::Wallet,
    types::{
        transaction::eip2718::TypedTransaction, Address, Block, BlockId, BlockNumber, Bytes,
        Filter, Log, NameOrAddress, Transaction, TransactionReceipt, TxHash, H256, U256, U64,
    },
};
use rand::thread_rng;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

use crate::environment::RevmEnvironment;

// can an agent be be a struct? well API wants users
#[derive(Debug)]
pub struct RevmMiddleware {
    // Provider/inner since this is the lowest level middleware
    inner: Provider<RevmEnvironment>,
    // Wallet
    pub wallet: Wallet<SigningKey>,
}

impl RevmMiddleware {
    pub fn new(environment: RevmEnvironment) -> Self {
        Self {
            inner: Provider::new(environment),
            wallet: Wallet::new(&mut thread_rng()),
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RevmMiddleware {
    /// The JSON-RPC client type at the bottom of the stack
    type Provider = RevmEnvironment;
    /// Error type returned by most operations
    type Error = ProviderError; //RevmMiddlewareError;
    /// The next-lower middleware in the middleware stack
    type Inner = Provider<RevmEnvironment>;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner.provider()
    }

    fn default_sender(&self) -> Option<Address> {
        todo!("This should be an Agent's address")
    }

    async fn get_block_number(&self) -> Result<U64, Self::Error> {
        todo!("we should be able to get the block number.")
    }

    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        tx: T,
        _block: Option<BlockId>,
    ) -> Result<PendingTransaction<'_, Self::Provider>, Self::Error> {
        // let tx: TypedTransaction = tx.into();

        // // Conversions
        // // let caller = B160::from(*tx.from().unwrap()).into();
        // let caller = B160::from_low_u64_be(1);
        // println!("caller: {:?}", caller);
        // let gas_limit = u64::MAX;
        // let gas_price: revm::primitives::U256 = revm::primitives::U256::from(0);
        // let bytes = tx.data().unwrap().clone().to_vec();
        // let data = bytes::Bytes::from(bytes);
        // let transact_to = match tx.to() {
        //     Some(to) => TransactTo::Call(B160::from(*to.as_address().unwrap()).into()),
        //     None => TransactTo::create(),
        // };
        // // tx.set_from(&self.eoa);

        // // Build the TxEnv
        // let tx_env = TxEnv {
        //     caller,
        //     gas_limit,
        //     gas_price,
        //     gas_priority_fee: None,
        //     transact_to,
        //     value: revm::primitives::U256::ZERO, // TODO: I doubt we want to ever send any raw eth to a contract.
        //     data,
        //     chain_id: None,
        //     nonce: None,
        //     access_list: Vec::new(),
        // };
        // let (sender, receiver) = crossbeam_channel::unbounded();
        // let _transaction = self.tx_sender.send((tx_env, sender));
        // let result = receiver.recv().unwrap();
        // println!("result: {:?}", result);
        let pending_tx = PendingTransaction::new(H256::default(), self.provider());
        Ok(pending_tx)
    }

    async fn get_block<T: Into<BlockId> + Send + Sync>(
        &self,
        _block_hash_or_number: T,
    ) -> Result<Option<Block<TxHash>>, Self::Error> {
        todo!("we should be able to get the block.")
    }

    async fn get_block_with_txs<T: Into<BlockId> + Send + Sync>(
        &self,
        _block_hash_or_number: T,
    ) -> Result<Option<Block<Transaction>>, Self::Error> {
        todo!("we should be able to get the block.")
    }

    async fn get_transaction_count<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<U256, Self::Error> {
        self.inner()
            .get_transaction_count(from, block)
            .await
            .map_err(MiddlewareError::from_err)
    }

    async fn call(
        &self,
        _tx: &TypedTransaction,
        _block: Option<BlockId>,
    ) -> Result<Bytes, Self::Error> {
        todo!("we should be able to call. We will have to consider adding a function to the `SimulationEnvironment` that uses `transact` and not `transact_commit`")
    }

    async fn get_balance<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<U256, Self::Error> {
        let thing = self.inner();
        self.inner()
            .get_balance(from, block)
            .await
            .map_err(MiddlewareError::from_err)
    }

    async fn get_transaction<T: Send + Sync + Into<TxHash>>(
        &self,
        _transaction_hash: T,
    ) -> Result<Option<Transaction>, Self::Error> {
        todo!("we should be able to get the transaction.")
    }

    async fn get_transaction_receipt<T: Send + Sync + Into<TxHash>>(
        &self,
        _transaction_hash: T,
    ) -> Result<Option<TransactionReceipt>, Self::Error> {
        todo!("we should be able to get the transaction receipt.")
    }

    async fn get_gas_price(&self) -> Result<U256, Self::Error> {
        todo!("low priority, but we should be able to set and get gas price for our environment.")
    }

    async fn get_logs(&self, _filter: &Filter) -> Result<Vec<Log>, Self::Error> {
        todo!("we should be able to get logs.")
    }

    async fn new_filter(&self, _filter: FilterKind<'_>) -> Result<U256, Self::Error> {
        todo!("we should be able to install a new filter.")
    }

    async fn uninstall_filter<T: Into<U256> + Send + Sync>(
        &self,
        _id: T,
    ) -> Result<bool, Self::Error> {
        todo!("we should be able to uninstall a filter.")
    }

    async fn watch<'a>(
        &'a self,
        _filter: &Filter,
    ) -> Result<FilterWatcher<'a, Self::Provider, Log>, Self::Error> {
        todo!("we should be able to watch. we already have this partially implemented for agents.")
    }

    async fn get_filter_changes<T, R>(&self, _id: T) -> Result<Vec<R>, Self::Error>
    where
        T: Into<U256> + Send + Sync,
        R: Serialize + DeserializeOwned + Send + Sync + Debug,
    {
        todo!("we should be able to get filter changes.")
    }

    async fn watch_blocks(&self) -> Result<FilterWatcher<'_, Self::Provider, H256>, Self::Error> {
        todo!("we should be able to watch blocks.")
    }

    async fn get_code<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        _at: T,
        _block: Option<BlockId>,
    ) -> Result<Bytes, Self::Error> {
        todo!("we should be able to get code.")
    }

    async fn get_storage_at<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        location: H256,
        block: Option<BlockId>,
    ) -> Result<H256, Self::Error> {
        self.inner()
            .get_storage_at(from, location, block)
            .await
            .map_err(MiddlewareError::from_err)
    }

    async fn import_raw_key(
        &self,
        _private_key: Bytes,
        _passphrase: String,
    ) -> Result<Address, Self::Error> {
        unimplemented!("we don't need to import raw keys.")
    }

    async fn unlock_account<T: Into<Address> + Send + Sync>(
        &self,
        _account: T,
        _passphrase: String,
        _duration: Option<u64>,
    ) -> Result<bool, Self::Error> {
        unimplemented!("we don't need to unlock accounts.")
    }

    async fn sign<T: Into<Bytes> + Send + Sync>(
        &self,
        data: T,
        from: &Address,
    ) -> Result<ethers::types::Signature, Self::Error> {
        self.inner()
            .sign(data, from)
            .await
            .map_err(MiddlewareError::from_err)
    }

    async fn sign_transaction(
        &self,
        tx: &TypedTransaction,
        from: Address,
    ) -> Result<ethers::types::Signature, Self::Error> {
        self.inner()
            .sign_transaction(tx, from)
            .await
            .map_err(MiddlewareError::from_err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_sender() {
        todo!("This should be an Agent's address")
    }

    async fn get_block_number() {
        todo!("we should be able to get the block number.")
    }

    async fn send_transaction() {
        todo!("we should be able to send transactions.")
    }

    async fn get_block() {
        todo!("we should be able to get the block.")
    }

    async fn get_block_with_txs() {
        todo!("we should be able to get the block.")
    }

    async fn get_transaction_count() {
        todo!("we should be able to get the transaction count.")
    }

    async fn call() {
        todo!("we should be able to call. We will have to consider adding a function to the `SimulationEnvironment` that uses `transact` and not `transact_commit`")
    }

    async fn get_balance() {
        todo!("we should be able to get the balance.")
    }

    async fn get_transaction() {
        todo!("we should be able to get the transaction.")
    }

    async fn get_transaction_receipt() {
        todo!("we should be able to get the transaction receipt.")
    }

    async fn get_gas_price() {
        todo!("low priority, but we should be able to set and get gas price for our environment.")
    }

    async fn get_logs() {
        todo!("we should be able to get logs.")
    }

    async fn new_filter() {
        todo!("we should be able to install a new filter.")
    }

    async fn uninstall_filter() {
        todo!("we should be able to uninstall a filter.")
    }

    async fn watch() {
        todo!("we should be able to watch. we already have this partially implemented for agents.")
    }

    async fn get_filter_changes() {
        todo!("we should be able to get filter changes.")
    }

    async fn watch_blocks() {
        todo!("we should be able to watch blocks.")
    }

    async fn get_code() {
        todo!("we should be able to get code.")
    }

    async fn get_storage_at() {}

    async fn import_raw_key() {
        unimplemented!("we don't need to import raw keys.")
    }

    async fn sign() {}

    async fn sign_transaction() {}
}
