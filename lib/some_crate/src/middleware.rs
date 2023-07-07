#![warn(missing_docs)]
#![allow(clippy::all)]
//! This module contains the middleware for the Revm simulation environment.
//! Most of the middleware is essentially a placeholder, but it is necessary to have a middleware to work with bindings more efficiently.

// TODO and notes:
// The middleware/client should be something like `AgentClient` or `AgentMiddleware`.
// The middleware/client needs to be able to send transactions from an address (and also have access to some specific channels and what not)

use std::fmt::Debug;

use crossbeam_channel::Sender;
use ethers::{
    prelude::k256::ecdsa::SigningKey,
    prelude::ProviderError,
    providers::{
        erc, FilterKind, FilterWatcher, LogQuery, Middleware, MiddlewareError, MockProvider,
        NodeInfo, PeerInfo, PendingTransaction, Provider,
    },
    signers::Wallet,
    types::{
        transaction::{eip2718::TypedTransaction, eip2930::AccessListWithGasUsed},
        Address, Block, BlockId, BlockNumber, BlockTrace, Bytes, EIP1186ProofResponse, FeeHistory,
        Filter, GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace, Log,
        NameOrAddress, SyncingStatus, Trace, TraceFilter, TraceType, Transaction,
        TransactionReceipt, TxHash, TxpoolContent, TxpoolInspect, TxpoolStatus, H256, U256, U64,
    },
};
use rand::thread_rng;
use revm::primitives::{ExecutionResult, TransactTo, TxEnv, B160};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

// can an agent be be a struct? well API wants users
#[derive(Debug)]
pub struct RevmMiddleware {
    /// tansaction sender
    pub tx_sender: Sender<(TxEnv, Sender<ExecutionResult>)>,
    // Provider
    pub provider: Provider<MockProvider>,
    // Wallet
    pub wallet: Wallet<SigningKey>,
}

impl RevmMiddleware {
    pub fn new(tx_sender: Sender<(TxEnv, Sender<ExecutionResult>)>) -> Self {
        Self {
            tx_sender,
            provider: ethers::providers::Provider::new(MockProvider::default()),
            wallet: Wallet::new(&mut thread_rng()),
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RevmMiddleware {
    /// The JSON-RPC client type at the bottom of the stack
    type Provider = MockProvider;
    /// Error type returned by most operations
    type Error = ProviderError;
    /// The next-lower middleware in the middleware stack
    type Inner = Self;

    /// Get a reference to the next-lower middleware in the middleware stack
    fn inner(&self) -> &Self::Inner {
        self
    }

    /// Return the default sender (if any). This will typically be the
    /// connected node's first address, or the address of a Signer in a lower
    /// middleware stack
    fn default_sender(&self) -> Option<Address> {
        todo!("This should be an Agent's address")
    }

    /// Get the block number
    async fn get_block_number(&self) -> Result<U64, Self::Error> {
        todo!("we should be able to get the block number.")
    }

    /// Sends the transaction to the entire Ethereum network and returns the
    /// transaction's hash. This will consume gas from the account that signed
    /// the transaction. This call will fail if no signer is available, and the
    /// RPC node does  not have an unlocked accounts
    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        tx: T,
        _block: Option<BlockId>,
    ) -> Result<PendingTransaction<'_, Self::Provider>, Self::Error> {
        let tx: TypedTransaction = tx.into();

        // Conversions
        // let caller = B160::from(*tx.from().unwrap()).into();
        let caller = B160::from_low_u64_be(1);
        println!("caller: {:?}", caller);
        let gas_limit = u64::MAX;
        let gas_price: revm::primitives::U256 = revm::primitives::U256::from(0);
        let bytes = tx.data().unwrap().clone().to_vec();
        let data = bytes::Bytes::from(bytes);
        let transact_to = match tx.to() {
            Some(to) => TransactTo::Call(B160::from(*to.as_address().unwrap()).into()),
            None => TransactTo::create(),
        };
        // tx.set_from(&self.eoa);

        // Build the TxEnv
        let tx_env = TxEnv {
            caller,
            gas_limit,
            gas_price,
            gas_priority_fee: None,
            transact_to,
            value: revm::primitives::U256::ZERO, // TODO: I doubt we want to ever send any raw eth to a contract.
            data,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        };
        let (sender, receiver) = crossbeam_channel::unbounded();
        let _transaction = self.tx_sender.send((tx_env, sender));
        let result = receiver.recv().unwrap();
        println!("result: {:?}", result);
        let pending_tx = PendingTransaction::new(H256::default(), self.provider());
        Ok(pending_tx)
    }

    /// Gets the block at `block_hash_or_number` (transaction hashes only)
    async fn get_block<T: Into<BlockId> + Send + Sync>(
        &self,
        _block_hash_or_number: T,
    ) -> Result<Option<Block<TxHash>>, Self::Error> {
        todo!("we should be able to get the block.")
    }

    /// Gets the block at `block_hash_or_number` (full transactions included)
    async fn get_block_with_txs<T: Into<BlockId> + Send + Sync>(
        &self,
        _block_hash_or_number: T,
    ) -> Result<Option<Block<Transaction>>, Self::Error> {
        todo!("we should be able to get the block.")
    }

    /// Returns the nonce of the address
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

    /// Sends the read-only (constant) transaction to a single Ethereum node and return the result
    /// (as bytes) of executing it. This is free, since it does not change any state on the
    /// blockchain.
    async fn call(
        &self,
        _tx: &TypedTransaction,
        _block: Option<BlockId>,
    ) -> Result<Bytes, Self::Error> {
        todo!("we should be able to call. We will have to consider adding a function to the `SimulationEnvironment` that uses `transact` and not `transact_commit`")
    }

    /// Returns the account's balance
    async fn get_balance<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        _from: T,
        _block: Option<BlockId>,
    ) -> Result<U256, Self::Error> {
        todo!("we should be able to get the balance.")
    }

    /// Gets the transaction with `transaction_hash`
    async fn get_transaction<T: Send + Sync + Into<TxHash>>(
        &self,
        _transaction_hash: T,
    ) -> Result<Option<Transaction>, Self::Error> {
        todo!("we should be able to get the transaction.")
    }

    /// Gets the transaction receipt with `transaction_hash`
    async fn get_transaction_receipt<T: Send + Sync + Into<TxHash>>(
        &self,
        _transaction_hash: T,
    ) -> Result<Option<TransactionReceipt>, Self::Error> {
        todo!("we should be able to get the transaction receipt.")
    }

    /// Returns all receipts for a block.
    ///
    /// Note that this uses the `eth_getBlockReceipts` RPC, which is
    /// non-standard and currently supported by Erigon.
    async fn get_block_receipts<T: Into<BlockNumber> + Send + Sync>(
        &self,
        _block: T,
    ) -> Result<Vec<TransactionReceipt>, Self::Error> {
        todo!("we should be able to get the block receipts.")
    }

    /// Gets the current gas price as estimated by the node
    async fn get_gas_price(&self) -> Result<U256, Self::Error> {
        todo!("low priority, but we should be able to set and get gas price for our environment.")
    }

    ////// Contract state

    /// Returns an array (possibly empty) of logs that match the filter
    async fn get_logs(&self, _filter: &Filter) -> Result<Vec<Log>, Self::Error> {
        todo!("we should be able to get logs.")
    }

    /// Returns a stream of logs are loaded in pages of given page size
    fn get_logs_paginated<'a>(
        &'a self,
        _filter: &Filter,
        _page_size: u64,
    ) -> LogQuery<'a, Self::Provider> {
        todo!("we should be able to get logs paginated.")
    }

    /// Install a new filter on the node.
    ///
    /// This method is hidden because filter lifecycle  should be managed by
    /// the [`FilterWatcher`]
    #[doc(hidden)]
    async fn new_filter(&self, _filter: FilterKind<'_>) -> Result<U256, Self::Error> {
        todo!("we should be able to install a new filter.")
    }

    /// Uninstalls a filter.
    ///
    /// This method is hidden because filter lifecycle  should be managed by
    /// the [`FilterWatcher`]
    #[doc(hidden)]
    async fn uninstall_filter<T: Into<U256> + Send + Sync>(
        &self,
        _id: T,
    ) -> Result<bool, Self::Error> {
        todo!("we should be able to uninstall a filter.")
    }

    /// Streams event logs matching the filter.
    ///
    /// This function streams via a polling system, by repeatedly dispatching
    /// RPC requests. If possible, prefer using a WS or IPC connection and the
    /// `stream` interface
    async fn watch<'a>(
        &'a self,
        _filter: &Filter,
    ) -> Result<FilterWatcher<'a, Self::Provider, Log>, Self::Error> {
        todo!("we should be able to watch. we already have this partially implemented for agents.")
    }

    /// Polling method for a filter, which returns an array of logs which occurred since last poll.
    ///
    /// This method must be called with one of the following return types, depending on the filter
    /// type:
    /// - `eth_newBlockFilter`: [`H256`], returns block hashes
    /// - `eth_newPendingTransactionFilter`: [`H256`], returns transaction hashes
    /// - `eth_newFilter`: [`Log`], returns raw logs
    ///
    /// If one of these types is not used, decoding will fail and the method will
    /// return an error.
    ///
    /// [`H256`]: ethers_core::types::H256
    /// [`Log`]: ethers_core::types::Log
    ///
    /// This method is hidden because filter lifecycle  should be managed by
    /// the [`FilterWatcher`]
    #[doc(hidden)]
    async fn get_filter_changes<T, R>(&self, _id: T) -> Result<Vec<R>, Self::Error>
    where
        T: Into<U256> + Send + Sync,
        R: Serialize + DeserializeOwned + Send + Sync + Debug,
    {
        todo!("we should be able to get filter changes.")
    }

    /// Streams new block hashes
    ///
    /// This function streams via a polling system, by repeatedly dispatching
    /// RPC requests. If possible, prefer using a WS or IPC connection and the
    /// `stream` interface
    async fn watch_blocks(&self) -> Result<FilterWatcher<'_, Self::Provider, H256>, Self::Error> {
        todo!("we should be able to watch blocks.")
    }

    /// Returns the deployed code at a given address
    async fn get_code<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        _at: T,
        _block: Option<BlockId>,
    ) -> Result<Bytes, Self::Error> {
        todo!("we should be able to get code.")
    }

    /// Get the storage of an address for a particular slot location
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

    // Personal namespace
    // NOTE: This will eventually need to be enabled by users explicitly because the personal
    // namespace is being deprecated:
    // Issue: https://github.com/ethereum/go-ethereum/issues/25948
    // PR: https://github.com/ethereum/go-ethereum/pull/26390

    /// Sends the given key to the node to be encrypted with the provided
    /// passphrase and stored.
    ///
    /// The key represents a secp256k1 private key and should be 32 bytes.
    async fn import_raw_key(
        &self,
        _private_key: Bytes,
        _passphrase: String,
    ) -> Result<Address, Self::Error> {
        unimplemented!("we don't need to import raw keys.")
    }

    /// Prompts the node to decrypt the given account from its keystore.
    ///
    /// If the duration provided is `None`, then the account will be unlocked
    /// indefinitely. Otherwise, the account will be unlocked for the provided
    /// number of seconds.
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
    use crate::agent::Agent;
    use anyhow;

    use crate::{bindings, environment::SimulationEnvironment};
    use bindings::writer::Writer;

    // #[tokio::test]
    // async fn test_something() -> anyhow::Result<()> {
    //     // TODO: Specify the signer to use here ::<SIGNER>.
    //     let environment = SimulationEnvironment::new();
    //     let thing = environment.event_broadcaster;
    //     let agent = Agent::new(crossbeam_channel::unbounded().0);

    //     let deployer = Writer::deploy(agent.client, ())?;
    //     println!("deployer: {:?}", deployer);
    //     let writer = deployer.send().await?;
    //     writer
    //         .echo_string(String::from("test_string"))
    //         .send()
    //         .await?;

    //     Ok(())
    // }
}
