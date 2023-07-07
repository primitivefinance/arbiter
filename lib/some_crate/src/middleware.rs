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
        unimplemented!("revm does not need any inner middleware.")
    }

    /// Convert a provider error into the associated error type by successively
    /// converting it to every intermediate middleware error
    fn convert_err(_p: ProviderError) -> Self::Error {
        unimplemented!("revm should not output any provider errors.")
    }

    /// The HTTP or Websocket provider.
    fn provider(&self) -> &Provider<Self::Provider> {
        &self.provider
    }

    /// Return the default sender (if any). This will typically be the
    /// connected node's first address, or the address of a Signer in a lower
    /// middleware stack
    fn default_sender(&self) -> Option<Address> {
        todo!("this may be the transaction channel.")
    }

    /// Returns the current client version using the `web3_clientVersion` RPC.
    async fn client_version(&self) -> Result<String, Self::Error> {
        todo!("we should be able to return the client version.")
    }

    /// Fill necessary details of a transaction for dispatch
    ///
    /// This function is defined on providers to behave as follows:
    /// 1. populate the `from` field with the default sender
    /// 2. resolve any ENS names in the tx `to` field
    /// 3. Estimate gas usage
    /// 4. Poll and set legacy or 1559 gas prices
    /// 5. Set the chain_id with the provider's, if not already set
    ///
    /// It does NOT set the nonce by default.
    ///
    /// Middleware are encouraged to override any values _before_ delegating
    /// to the inner implementation AND/OR modify the values provided by the
    /// default implementation _after_ delegating.
    ///
    /// E.g. a middleware wanting to double gas prices should consider doing so
    /// _after_ delegating and allowing the default implementation to poll gas.
    async fn fill_transaction(
        &self,
        _tx: &mut TypedTransaction,
        _block: Option<BlockId>,
    ) -> Result<(), Self::Error> {
        todo!("we should be able to fill the transaction.")
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

    /// Send a transaction with a simple escalation policy.
    ///
    /// `policy` should be a boxed function that maps `original_gas_price`
    /// and `number_of_previous_escalations` -> `new_gas_price`.
    ///
    /// e.g. `Box::new(|start, escalation_index| start * 1250.pow(escalations) /
    /// 1000.pow(escalations))`
    // async fn send_escalating<'a>(
    //     &'a self,
    //     tx: &TypedTransaction,
    //     escalations: usize,
    //     policy: ethers::providers::EscalationPolicy,
    // ) -> Result<ethers::providers::EscalatingPending<'a,Self::Provider> , Self::Error> {
    //     unimplemented!("revm does not need to escalate transactions at the moment.")
    // }

    ////// Ethereum Naming Service
    // The Ethereum Naming Service (ENS) allows easy to remember and use names to
    // be assigned to Ethereum addresses. Any provider operation which takes an address
    // may also take an ENS name.
    //
    // ENS also provides the ability for a reverse lookup, which determines the name for an address
    // if it has been configured.

    /// Returns the address that the `ens_name` resolves to (or None if not configured).
    ///
    /// # Panics
    ///
    /// If the bytes returned from the ENS registrar/resolver cannot be interpreted as
    /// an address. This should theoretically never happen.
    async fn resolve_name(&self, _ens_name: &str) -> Result<Address, Self::Error> {
        unimplemented!("revm does not need ens at the moment.")
    }

    /// Returns the ENS name the `address` resolves to (or None if not configured).
    ///
    /// # Panics
    ///
    /// If the bytes returned from the ENS registrar/resolver cannot be interpreted as
    /// a string. This should theoretically never happen.
    async fn lookup_address(&self, _address: Address) -> Result<String, Self::Error> {
        unimplemented!("revm does not need ens at the moment.")
    }

    /// Returns the avatar HTTP link of the avatar that the `ens_name` resolves to (or None
    /// if not configured)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ethers_providers::{Provider, Http, Middleware};
    /// # async fn foo(provider: Provider<Http>) -> Result<(), Box<dyn std::error::Error>> {
    /// let avatar = provider.resolve_avatar("parishilton.eth").await?;
    /// assert_eq!(avatar.to_string(), "https://i.imgur.com/YW3Hzph.jpg");
    /// # Ok(()) }
    /// ```
    ///
    /// # Panics
    ///
    /// If the bytes returned from the ENS registrar/resolver cannot be interpreted as
    /// a string. This should theoretically never happen.
    async fn resolve_avatar(&self, _ens_name: &str) -> Result<Url, Self::Error> {
        unimplemented!("revm does not need ens at the moment.")
    }

    /// Returns the URL (not necesserily HTTP) of the image behind a token.
    ///
    /// # Example
    /// ```no_run
    /// # use ethers_providers::{Provider, Http, Middleware};
    /// use ethers_providers::erc::ERCNFT;
    /// # async fn foo(provider: Provider<Http>) -> Result<(), Box<dyn std::error::Error>> {
    /// let token = "erc721:0xc92ceddfb8dd984a89fb494c376f9a48b999aafc/9018".parse()?;
    /// let token_image = provider.resolve_nft(token).await?;
    /// assert_eq!(
    ///     token_image.to_string(),
    ///     "https://creature.mypinata.cloud/ipfs/QmNwj3aUzXfG4twV3no7hJRYxLLAWNPk6RrfQaqJ6nVJFa/9018.jpg"
    /// );
    /// # Ok(()) }
    /// ```
    ///
    /// # Panics
    ///
    /// If the bytes returned from the ENS registrar/resolver cannot be interpreted as
    /// a string. This should theoretically never happen.
    async fn resolve_nft(&self, _token: erc::ERCNFT) -> Result<Url, Self::Error> {
        unimplemented!("revm does not need nfts at the moment.")
    }

    /// Fetch a field for the `ens_name` (no None if not configured).
    ///
    /// # Panics
    ///
    /// If the bytes returned from the ENS registrar/resolver cannot be interpreted as
    /// a string. This should theoretically never happen.
    async fn resolve_field(&self, _ens_name: &str, _field: &str) -> Result<String, Self::Error> {
        unimplemented!("revm does not need ens at the moment.")
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

    /// Gets the block uncle count at `block_hash_or_number`
    async fn get_uncle_count<T: Into<BlockId> + Send + Sync>(
        &self,
        _block_hash_or_number: T,
    ) -> Result<U256, Self::Error> {
        unimplemented!("revm does not need uncles at the moment.")
    }

    /// Gets the block uncle at `block_hash_or_number` and `idx`
    async fn get_uncle<T: Into<BlockId> + Send + Sync>(
        &self,
        _block_hash_or_number: T,
        _idx: U64,
    ) -> Result<Option<Block<H256>>, Self::Error> {
        unimplemented!("revm does not need uncles at the moment.")
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

    /// Sends a transaction to a single Ethereum node and return the estimated amount of gas
    /// required (as a U256) to send it This is free, but only an estimate. Providing too little
    /// gas will result in a transaction being rejected (while still consuming all provided
    /// gas).
    async fn estimate_gas(
        &self,
        tx: &TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<U256, Self::Error> {
        self.inner()
            .estimate_gas(tx, block)
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

    /// Return current client syncing status. If IsFalse sync is over.
    async fn syncing(&self) -> Result<SyncingStatus, Self::Error> {
        unimplemented!("revm does not need syncing at the moment.")
    }

    /// Returns the currently configured chain id, a value used in replay-protected
    /// transaction signing as introduced by EIP-155.
    async fn get_chainid(&self) -> Result<U256, Self::Error> {
        unimplemented!("revm does not need chainid at the moment.")
    }

    /// Returns the network version.
    async fn get_net_version(&self) -> Result<String, Self::Error> {
        unimplemented!("revm does not need net_version at the moment.")
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

    /// Gets a heuristic recommendation of max fee per gas and max priority fee per gas for
    /// EIP-1559 compatible transactions.
    async fn estimate_eip1559_fees(
        &self,
        _estimator: Option<fn(U256, Vec<Vec<U256>>) -> (U256, U256)>,
    ) -> Result<(U256, U256), Self::Error> {
        todo!("low priority, but we should be able to set and get gas price for our environment.")
    }

    /// Gets the accounts on the node
    async fn get_accounts(&self) -> Result<Vec<Address>, Self::Error> {
        todo!("we should be able to get the accounts.")
    }

    /// Send the raw RLP encoded transaction to the entire Ethereum network and returns the
    /// transaction's hash This will consume gas from the account that signed the transaction.
    async fn send_raw_transaction<'a>(
        &'a self,
        _tx: Bytes,
    ) -> Result<PendingTransaction<'a, Self::Provider>, Self::Error> {
        todo!("we should be able to send raw transactions.")
    }

    /// This returns true if either the middleware stack contains a `SignerMiddleware`, or the
    /// JSON-RPC provider has an unlocked key that can sign using the `eth_sign` call. If none of
    /// the above conditions are met, then the middleware stack is not capable of signing data.
    async fn is_signer(&self) -> bool {
        unimplemented!(
            "should we be able to check if we are a signer (I'm not sure how this works)."
        )
    }

    /// Signs data using a specific account. This account needs to be unlocked,
    /// or the middleware stack must contain a `SignerMiddleware`
    // async fn sign<T: Into<Bytes> + Send + Sync>(
    //     &self,
    //     data: T,
    //     from: &Address,
    // ) -> Result<Signature, Self::Error> {
    //     unimplemented!("should we be able to sign data (I'm not sure how this works).")
    // }

    /// Sign a transaction via RPC call
    // async fn sign_transaction(
    //     &self,
    //     tx: &TypedTransaction,
    //     from: Address,
    // ) -> Result<Signature, Self::Error> {
    //     unimplemented!("should we be able to sign transactions (I'm not sure how this works).")
    // }

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

    /// Streams pending transactions.
    ///
    /// This function streams via a polling system, by repeatedly dispatching
    /// RPC requests. If possible, prefer using a WS or IPC connection and the
    /// `stream` interface
    async fn watch_pending_transactions(
        &self,
    ) -> Result<FilterWatcher<'_, Self::Provider, H256>, Self::Error> {
        unimplemented!("we should not have to watch pending transactions.")
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

    /// Returns the EIP-1186 proof response
    /// <https://github.com/ethereum/EIPs/issues/1186>
    async fn get_proof<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        _from: T,
        _locations: Vec<H256>,
        _block: Option<BlockId>,
    ) -> Result<EIP1186ProofResponse, Self::Error> {
        unimplemented!("we don't need to get proofs.")
    }

    /// Returns an indication if this node is currently mining.
    async fn mining(&self) -> Result<bool, Self::Error> {
        unimplemented!("there is no node to be mining.")
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

    // async fn subscribe<T, R>(
    //     &self,
    //     params: T,
    // ) -> Result<ethers::providers::SubscriptionStream<'_, Self::Provider, R>, Self::Error>
    // where
    //     T: Debug + Serialize + Send + Sync,
    //     R: DeserializeOwned + Send + Sync,
    //     <Self as Middleware>::Provider: ethers::providers::PubsubClient,
    // {
    //     todo!("we will need to subscribe to something soon.")
    // }

    // async fn unsubscribe<T>(&self, id: T) -> Result<bool, Self::Error>
    // where
    //     T: Into<U256> + Send + Sync,
    //     <Self as Middleware>::Provider: ethers::providers::PubsubClient,
    // {
    //     todo!("we will need to unsubscribe from something soon.")
    // }

    // async fn subscribe_blocks(
    //     &self,
    // ) -> Result<ethers::providers::SubscriptionStream<'_, Self::Provider, Block<TxHash>>, Self::Error>
    // where
    //     <Self as Middleware>::Provider: ethers::providers::PubsubClient,
    // {
    //     todo!("we will need to subscribe to blocks soon.")
    // }

    // async fn subscribe_pending_txs(
    //     &self,
    // ) -> Result<ethers::providers::SubscriptionStream<'_, Self::Provider, TxHash>, Self::Error>
    // where
    //     <Self as Middleware>::Provider: ethers::providers::PubsubClient,
    // {
    //     todo!("we will need to subscribe to pending txs soon.")
    // }

    // async fn subscribe_logs<'a>(
    //     &'a self,
    //     filter: &Filter,
    // ) -> Result<ethers::providers::SubscriptionStream<'a, Self::Provider, Log>, Self::Error>
    // where
    //     <Self as Middleware>::Provider: ethers::providers::PubsubClient,
    // {
    //     todo!("we will need to subscribe to logs soon.")
    // }
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
