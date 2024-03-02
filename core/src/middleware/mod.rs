use alloy::primitives::{BlockHash, StorageKey, StorageValue, TxHash, U64};
use alloy_providers::{provider::TempProvider, utils::EstimatorFunction};
use alloy_rpc_trace_types::{
    geth::{GethDebugTracingOptions, GethTrace},
    parity::LocalizedTransactionTrace,
};
use alloy_rpc_types::{
    request::TransactionRequest, state::StateOverride, AccessListWithGasUsed, Block, BlockId,
    BlockNumberOrTag, EIP1186AccountProofResponse, FeeHistory, Filter, SyncStatus, Transaction,
    TransactionReceipt,
};
use alloy_transport::TransportResult;
use serde::de::DeserializeOwned;

use self::database::inspector::ArbiterInspector;

use super::*;

/// Represents a connection to the EVM contained in the corresponding
/// [`Environment`].
#[derive(Debug)]
pub struct Connection<'a> {
    /// Used to send calls and transactions to the [`Environment`] to be
    /// executed by `revm`.
    pub(crate) evm: Arc<RwLock<Evm<'a, ArbiterInspector, ArbiterDB>>>,
}

#[async_trait::async_trait]
impl TempProvider for Connection<'_> {
    /// Gets the transaction count of the corresponding address.
    async fn get_transaction_count(
        &self,
        address: Address,
        tag: Option<BlockId>,
    ) -> TransportResult<U256> {
        todo!()
    }

    /// Gets the last block number available.
    async fn get_block_number(&self) -> TransportResult<u64> {
        todo!()
    }

    /// Gets the balance of the account at the specified tag, which defaults to latest.
    async fn get_balance(&self, address: Address, tag: Option<BlockId>) -> TransportResult<U256> {
        todo!()
    }

    /// Gets a block by either its hash, tag, or number, with full transactions or only hashes.
    async fn get_block(&self, id: BlockId, full: bool) -> TransportResult<Option<Block>> {
        match id {
            BlockId::Hash(hash) => self.get_block_by_hash(hash.into(), full).await,
            BlockId::Number(number) => self.get_block_by_number(number, full).await,
        }
    }

    /// Gets a block by its [BlockHash], with full transactions or only hashes.
    async fn get_block_by_hash(
        &self,
        hash: BlockHash,
        full: bool,
    ) -> TransportResult<Option<Block>> {
        todo!()
    }

    /// Gets a block by [BlockNumberOrTag], with full transactions or only hashes.
    async fn get_block_by_number(
        &self,
        number: BlockNumberOrTag,
        full: bool,
    ) -> TransportResult<Option<Block>> {
        todo!()
    }

    /// Gets the client version of the chain client.
    async fn get_client_version(&self) -> TransportResult<String> {
        todo!()
    }

    /// Gets the chain ID.
    async fn get_chain_id(&self) -> TransportResult<U64> {
        todo!()
    }

    /// Gets the network ID. Same as `eth_chainId`.
    async fn get_net_version(&self) -> TransportResult<U64> {
        todo!()
    }

    /// Gets the specified storage value from [Address].
    async fn get_storage_at(
        &self,
        address: Address,
        key: U256,
        tag: Option<BlockId>,
    ) -> TransportResult<StorageValue> {
        todo!()
    }

    /// Gets the bytecode located at the corresponding [Address].
    async fn get_code_at(&self, address: Address, tag: Option<BlockId>) -> TransportResult<Bytes> {
        todo!()
    }

    /// Gets a [Transaction] by its [TxHash].
    async fn get_transaction_by_hash(&self, hash: TxHash) -> TransportResult<Transaction> {
        todo!()
    }

    /// Retrieves a [`Vec<Log>`] with the given [Filter].
    async fn get_logs(&self, filter: Filter) -> TransportResult<Vec<Log>> {
        todo!()
    }

    /// Gets the accounts in the remote node. This is usually empty unless you're using a local
    /// node.
    async fn get_accounts(&self) -> TransportResult<Vec<Address>> {
        todo!()
    }

    /// Gets the current gas price.
    async fn get_gas_price(&self) -> TransportResult<U256> {
        todo!()
    }

    /// Gets a [TransactionReceipt] if it exists, by its [TxHash].
    async fn get_transaction_receipt(
        &self,
        hash: TxHash,
    ) -> TransportResult<Option<TransactionReceipt>> {
        todo!()
    }

    /// Returns a collection of historical gas information [FeeHistory] which
    /// can be used to calculate the EIP1559 fields `maxFeePerGas` and `maxPriorityFeePerGas`.
    async fn get_fee_history(
        &self,
        block_count: U256,
        last_block: BlockNumberOrTag,
        reward_percentiles: &[f64],
    ) -> TransportResult<FeeHistory> {
        todo!()
    }

    /// Gets the selected block [BlockNumberOrTag] receipts.
    async fn get_block_receipts(
        &self,
        block: BlockNumberOrTag,
    ) -> TransportResult<Option<Vec<TransactionReceipt>>> {
        todo!()
    }

    /// Gets an uncle block through the tag [BlockId] and index [U64].
    async fn get_uncle(&self, tag: BlockId, idx: U64) -> TransportResult<Option<Block>> {
        todo!()
    }

    /// Gets syncing info.
    async fn syncing(&self) -> TransportResult<SyncStatus> {
        todo!()
    }

    /// Execute a smart contract call with [TransactionRequest] without publishing a transaction.
    async fn call(&self, tx: TransactionRequest, block: Option<BlockId>) -> TransportResult<Bytes> {
        todo!()
    }

    /// Execute a smart contract call with [TransactionRequest] and state overrides, without
    /// publishing a transaction.
    ///
    /// # Note
    ///
    /// Not all client implementations support state overrides.
    async fn call_with_overrides(
        &self,
        tx: TransactionRequest,
        block: Option<BlockId>,
        state: StateOverride,
    ) -> TransportResult<Bytes> {
        todo!()
    }

    /// Estimate the gas needed for a transaction.
    async fn estimate_gas(
        &self,
        tx: TransactionRequest,
        block: Option<BlockId>,
    ) -> TransportResult<U256> {
        todo!()
    }

    /// Sends an already-signed transaction.
    async fn send_raw_transaction(&self, tx: Bytes) -> TransportResult<TxHash> {
        todo!()
    }

    /// Estimates the EIP1559 `maxFeePerGas` and `maxPriorityFeePerGas` fields.
    /// Receives an optional [EstimatorFunction] that can be used to modify
    /// how to estimate these fees.
    async fn estimate_eip1559_fees(
        &self,
        estimator: Option<EstimatorFunction>,
    ) -> TransportResult<(U256, U256)> {
        todo!()
    }

    #[cfg(feature = "anvil")]
    async fn set_code(&self, address: Address, code: &'static str) -> TransportResult<()>;

    async fn get_proof(
        &self,
        address: Address,
        keys: Vec<StorageKey>,
        block: Option<BlockId>,
    ) -> TransportResult<EIP1186AccountProofResponse> {
        todo!()
    }

    async fn create_access_list(
        &self,
        request: TransactionRequest,
        block: Option<BlockId>,
    ) -> TransportResult<AccessListWithGasUsed> {
        todo!()
    }

    /// Parity trace transaction.
    async fn trace_transaction(
        &self,
        hash: TxHash,
    ) -> TransportResult<Vec<LocalizedTransactionTrace>> {
        todo!()
    }

    async fn debug_trace_transaction(
        &self,
        hash: TxHash,
        trace_options: GethDebugTracingOptions,
    ) -> TransportResult<GethTrace> {
        todo!()
    }

    async fn trace_block(
        &self,
        block: BlockNumberOrTag,
    ) -> TransportResult<Vec<LocalizedTransactionTrace>> {
        todo!()
    }

    async fn raw_request<P, R>(&self, method: &'static str, params: P) -> TransportResult<R>
    where
        P: Serialize + Send + Sync + Clone,
        R: Serialize + DeserializeOwned + Send + Sync + Unpin + 'static,
        Self: Sync,
    {
        todo!()
    }
}
