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

    // Admin namespace

    /// Requests adding the given peer, returning a boolean representing
    /// whether or not the peer was accepted for tracking.
    async fn add_peer(&self, _enode_url: String) -> Result<bool, Self::Error> {
        unimplemented!("we don't need to add peers.")
    }

    /// Requests adding the given peer as a trusted peer, which the node will
    /// always connect to even when its peer slots are full.
    async fn add_trusted_peer(&self, _enode_url: String) -> Result<bool, Self::Error> {
        unimplemented!("we don't need to add trusted peers.")
    }

    /// Returns general information about the node as well as information about the running p2p
    /// protocols (e.g. `eth`, `snap`).
    async fn node_info(&self) -> Result<NodeInfo, Self::Error> {
        unimplemented!("we don't need to get node info.")
    }

    /// Returns the list of peers currently connected to the node.
    async fn peers(&self) -> Result<Vec<PeerInfo>, Self::Error> {
        unimplemented!("we don't need to get peers.")
    }

    /// Requests to remove the given peer, returning true if the enode was successfully parsed and
    /// the peer was removed.
    async fn remove_peer(&self, _enode_url: String) -> Result<bool, Self::Error> {
        unimplemented!("we don't need to remove peers.")
    }

    /// Requests to remove the given peer, returning a boolean representing whether or not the
    /// enode url passed was validated. A return value of `true` does not necessarily mean that the
    /// peer was disconnected.
    async fn remove_trusted_peer(&self, _enode_url: String) -> Result<bool, Self::Error> {
        unimplemented!("we don't need to remove trusted peers.")
    }

    // Miner namespace

    /// Starts the miner with the given number of threads. If threads is nil, the number of workers
    /// started is equal to the number of logical CPUs that are usable by this process. If mining
    /// is already running, this method adjust the number of threads allowed to use and updates the
    /// minimum price required by the transaction pool.
    async fn start_mining(&self) -> Result<(), Self::Error> {
        unimplemented!("we don't need to start mining.")
    }

    /// Stop terminates the miner, both at the consensus engine level as well as at
    /// the block creation level.
    async fn stop_mining(&self) -> Result<(), Self::Error> {
        unimplemented!("we don't need to stop mining.")
    }

    // Mempool inspection for Geth's API

    /// Returns the details of all transactions currently pending for inclusion in the next
    /// block(s), as well as the ones that are being scheduled for future execution only.
    /// Ref: [Here](https://geth.ethereum.org/docs/rpc/ns-txpool#txpool_content)
    async fn txpool_content(&self) -> Result<TxpoolContent, Self::Error> {
        todo!("we should have a mempool, but we don't yet.")
    }

    /// Returns a summary of all the transactions currently pending for inclusion in the next
    /// block(s), as well as the ones that are being scheduled for future execution only.
    /// Ref: [Here](https://geth.ethereum.org/docs/rpc/ns-txpool#txpool_inspect)
    async fn txpool_inspect(&self) -> Result<TxpoolInspect, Self::Error> {
        todo!("we should have a mempool, but we don't yet.")
    }

    /// Returns the number of transactions currently pending for inclusion in the next block(s), as
    /// well as the ones that are being scheduled for future execution only.
    /// Ref: [Here](https://geth.ethereum.org/docs/rpc/ns-txpool#txpool_status)
    async fn txpool_status(&self) -> Result<TxpoolStatus, Self::Error> {
        todo!("we should have a mempool, but we don't yet.")
    }

    // Geth `trace` support

    /// After replaying any previous transactions in the same block,
    /// Replays a transaction, returning the traces configured with passed options
    async fn debug_trace_transaction(
        &self,
        _tx_hash: TxHash,
        _trace_options: GethDebugTracingOptions,
    ) -> Result<GethTrace, Self::Error> {
        unimplemented!("we don't need to debug transaction traces yet.")
    }

    /// Executes the given call and returns a number of possible traces for it
    async fn debug_trace_call<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        _req: T,
        _block: Option<BlockId>,
        _trace_options: GethDebugTracingCallOptions,
    ) -> Result<GethTrace, Self::Error> {
        unimplemented!("we don't need to debug call traces yet.")
    }

    /// Replays all transactions in a given block (specified by block number) and returns the traces
    /// configured with passed options
    /// Ref:
    /// [Here](https://geth.ethereum.org/docs/interacting-with-geth/rpc/ns-debug#debugtraceblockbynumber)
    async fn debug_trace_block_by_number(
        &self,
        _block: Option<BlockNumber>,
        _trace_options: GethDebugTracingOptions,
    ) -> Result<Vec<GethTrace>, Self::Error> {
        unimplemented!("we don't need to debug block traces yet.")
    }

    /// Replays all transactions in a given block (specified by block hash) and returns the traces
    /// configured with passed options
    /// Ref:
    /// [Here](https://geth.ethereum.org/docs/interacting-with-geth/rpc/ns-debug#debugtraceblockbyhash)
    async fn debug_trace_block_by_hash(
        &self,
        _block: H256,
        _trace_options: GethDebugTracingOptions,
    ) -> Result<Vec<GethTrace>, Self::Error> {
        unimplemented!("we don't need to debug block traces yet.")
    }

    // Parity `trace` support

    /// Executes the given call and returns a number of possible traces for it
    async fn trace_call<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        _req: T,
        _trace_type: Vec<TraceType>,
        _block: Option<BlockNumber>,
    ) -> Result<BlockTrace, Self::Error> {
        unimplemented!("we don't need to trace calls yet.")
    }

    /// Executes given calls and returns a number of possible traces for each
    /// call
    async fn trace_call_many<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        _req: Vec<(T, Vec<TraceType>)>,
        _block: Option<BlockNumber>,
    ) -> Result<Vec<BlockTrace>, Self::Error> {
        unimplemented!("we don't need to trace calls yet.")
    }

    /// Traces a call to `eth_sendRawTransaction` without making the call, returning the traces
    async fn trace_raw_transaction(
        &self,
        _data: Bytes,
        _trace_type: Vec<TraceType>,
    ) -> Result<BlockTrace, Self::Error> {
        unimplemented!("we don't need to trace raw transactions yet.")
    }

    /// Replays a transaction, returning the traces
    async fn trace_replay_transaction(
        &self,
        _hash: H256,
        _trace_type: Vec<TraceType>,
    ) -> Result<BlockTrace, Self::Error> {
        unimplemented!("we don't need to trace transactions yet.")
    }

    /// Replays all transactions in a block returning the requested traces for each transaction
    async fn trace_replay_block_transactions(
        &self,
        _block: BlockNumber,
        _trace_type: Vec<TraceType>,
    ) -> Result<Vec<BlockTrace>, Self::Error> {
        unimplemented!("we don't need to trace transactions yet.")
    }

    /// Returns traces created at given block
    async fn trace_block(&self, _block: BlockNumber) -> Result<Vec<Trace>, Self::Error> {
        unimplemented!("we don't need to trace blocks yet.")
    }

    /// Return traces matching the given filter
    async fn trace_filter(&self, _filter: TraceFilter) -> Result<Vec<Trace>, Self::Error> {
        unimplemented!("we don't need to trace filters yet.")
    }

    /// Returns trace at the given position
    async fn trace_get<T: Into<U64> + Send + Sync>(
        &self,
        _hash: H256,
        _index: Vec<T>,
    ) -> Result<Trace, Self::Error> {
        unimplemented!("we don't need to trace get yet.")
    }

    /// Returns all traces of a given transaction
    async fn trace_transaction(&self, _hash: H256) -> Result<Vec<Trace>, Self::Error> {
        unimplemented!("we don't need to trace transactions yet.")
    }

    // Parity namespace

    /// Returns all receipts for that block. Must be done on a parity node.
    async fn parity_block_receipts<T: Into<BlockNumber> + Send + Sync>(
        &self,
        _block: T,
    ) -> Result<Vec<TransactionReceipt>, Self::Error> {
        unimplemented!("we don't need to get block receipts yet.")
    }

    /// Create a new subscription
    ///
    /// This method is hidden as subscription lifecycles are intended to be
    /// handled by a [`SubscriptionStream`] object.
    // #[doc(hidden)]
    // async fn subscribe<T, R>(
    //     &self,
    //     params: T,
    // ) -> Result<SubscriptionStream<'_, Self::Provider, R>, Self::Error>
    // where
    //     T: Debug + Serialize + Send + Sync,
    //     R: DeserializeOwned + Send + Sync,
    //     <Self as Middleware>::Provider: PubsubClient,
    // {
    //     todo!("we will need to subscribe to events soon.")
    // }

    /// Instruct the RPC to cancel a subscription by its ID
    ///
    /// This method is hidden as subscription lifecycles are intended to be
    /// handled by a [`SubscriptionStream`] object
    // #[doc(hidden)]
    // async fn unsubscribe<T>(&self, id: T) -> Result<bool, Self::Error>
    // where
    //     T: Into<U256> + Send + Sync,
    //     <Self as Middleware>::Provider: PubsubClient,
    // {
    //     todo!("we will need to unsubscribe to events soon.")
    // }

    /// Subscribe to a stream of incoming blocks.
    ///
    /// This function is only available on pubsub clients, such as Websockets
    /// or IPC. For a polling alternative available over HTTP, use
    /// [`Middleware::watch_blocks`]. However, be aware that polling increases
    /// RPC usage drastically.
    // async fn subscribe_blocks(
    //     &self,
    // ) -> Result<SubscriptionStream<'_, Self::Provider, Block<TxHash>>, Self::Error>
    // where
    //     <Self as Middleware>::Provider: PubsubClient,
    // {
    //     todo!("we will need to subscribe to blocks soon.")
    // }

    /// Subscribe to a stream of pending transactions.
    ///
    /// This function is only available on pubsub clients, such as Websockets
    /// or IPC. For a polling alternative available over HTTP, use
    /// [`Middleware::watch_pending_transactions`]. However, be aware that
    /// polling increases RPC usage drastically.
    // async fn subscribe_pending_txs(
    //     &self,
    // ) -> Result<SubscriptionStream<'_, Self::Provider, TxHash>, Self::Error>
    // where
    //     <Self as Middleware>::Provider: PubsubClient,
    // {
    //     todo!("we will need to subscribe to pending transactions soon, if we have a TxPool.")
    // }

    /// Subscribe to a stream of event logs matchin the provided [`Filter`].
    ///
    /// This function is only available on pubsub clients, such as Websockets
    /// or IPC. For a polling alternative available over HTTP, use
    /// [`Middleware::watch`]. However, be aware that polling increases
    /// RPC usage drastically.
    // async fn subscribe_logs<'a>(
    //     &'a self,
    //     filter: &Filter,
    // ) -> Result<SubscriptionStream<'a, Self::Provider, Log>, Self::Error>
    // where
    //     <Self as Middleware>::Provider: PubsubClient,
    // {
    //     todo!("we will need to subscribe to logs soon.")
    // }

    /// Query the node for a [`FeeHistory`] object. This objct contains
    /// information about the EIP-1559 base fee in past blocks, as well as gas
    /// utilization within those blocks.
    ///
    /// See the
    /// [EIP-1559 documentation](https://eips.ethereum.org/EIPS/eip-1559) for
    /// details
    async fn fee_history<T: Into<U256> + serde::Serialize + Send + Sync>(
        &self,
        _block_count: T,
        _last_block: BlockNumber,
        _reward_percentiles: &[f64],
    ) -> Result<FeeHistory, Self::Error> {
        unimplemented!("we don't need to get fee history yet.")
    }

    /// Querty the node for an EIP-2930 Access List.
    ///
    /// See the
    /// [EIP-2930 documentation](https://eips.ethereum.org/EIPS/eip-2930) for
    /// details
    async fn create_access_list(
        &self,
        _tx: &TypedTransaction,
        _block: Option<BlockId>,
    ) -> Result<AccessListWithGasUsed, Self::Error> {
        unimplemented!("we don't need to create access lists yet.")
    }

    async fn send_escalating<'a>(
        &'a self,
        tx: &TypedTransaction,
        escalations: usize,
        policy: ethers::providers::EscalationPolicy,
    ) -> Result<ethers::providers::EscalatingPending<'a, Self::Provider>, Self::Error> {
        // let mut original = tx.clone();
        // self.fill_transaction(&mut original, None).await?;

        // // set the nonce, if no nonce is found
        // if original.nonce().is_none() {
        //     let nonce =
        //         self.get_transaction_count(tx.from().copied().unwrap_or_default(), None).await?;
        //     original.set_nonce(nonce);
        // }

        // let gas_price = original.gas_price().expect("filled");
        // let sign_futs: Vec<_> = (0..escalations)
        //     .map(|i| {
        //         let new_price = policy(gas_price, i);
        //         let mut r = original.clone();
        //         r.set_gas_price(new_price);
        //         r
        //     })
        //     .map(|req| async move {
        //         self.sign_transaction(&req, self.default_sender().unwrap_or_default())
        //             .await
        //             .map(|sig| req.rlp_signed(&sig))
        //     })
        //     .collect();

        // // we reverse for convenience. Ensuring that we can always just
        // // `pop()` the next tx off the back later
        // let mut signed = futures::future::join_all(sign_futs).await.into_iter().collect::<Result<Vec<_>, _>>()?;
        // signed.reverse();

        // Ok(ethers::providers::EscalatingPending::new(self.provider(), signed))
        todo!("do this")
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

    #[tokio::test]
    async fn test_something() -> anyhow::Result<()> {
        // TODO: Specify the signer to use here ::<SIGNER>.
        let env = SimulationEnvironment::new();

        let agent = Agent::new(crossbeam_channel::unbounded().0);

        let deployer = Writer::deploy(agent.client, ())?;
        println!("deployer: {:?}", deployer);
        let writer = deployer.send().await?;
        writer
            .echo_string(String::from("test_string"))
            .send()
            .await?;

        Ok(())
    }
}
