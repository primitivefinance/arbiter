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
    providers::{FilterWatcher, Middleware},
    signers::{Signer, Wallet},
    types::{transaction::eip2718::TypedTransaction, Address, BlockId, Bytes, Filter, Log},
};
use ethers_middleware::providers::MockProvider;
use rand::{thread_rng, SeedableRng};
use rand::rngs::StdRng;
use revm::primitives::{TransactTo, TxEnv, B160, U256};
use std::fmt::Debug;

use crate::environment::Connection;

// can an agent be be a struct? well API wants users
#[derive(Debug)]
pub struct RevmMiddleware {
    connection: Connection,
    wallet: Wallet<SigningKey>,
}

// We can let an agent create their own middleware that has their own environment
impl RevmMiddleware {
    pub(crate) fn new(connection: Connection) -> Self {
        let mut rng = StdRng::from_seed([1; 32]);
        Self {
            connection,
            wallet: Wallet::new(&mut rng),
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

    fn default_sender(&self) -> Option<Address> {
        Some(self.wallet.address())
    }

    async fn fill_transaction(
        &self,
        tx: &mut TypedTransaction,
        _block: Option<BlockId>,
    ) -> Result<(), Self::Error> {
        // tx.set_from(self.address());
        println!("the tx is: {:?}", tx);
        let ethers_bytes = tx.data().unwrap().clone();
        let bytes = bytes::Bytes::from(ethers_bytes.to_vec());
        let _tx_env = TxEnv {
            caller: B160::from(*tx.from().unwrap()),
            gas_limit: u64::MAX,
            gas_price: U256::ZERO,
            gas_priority_fee: None,
            transact_to: TransactTo::Call(B160::from(*tx.to_addr().unwrap())),
            value: U256::ZERO,
            data: bytes,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        };
        Ok(())
    }

    async fn call(
        &self,
        _tx: &TypedTransaction,
        _block: Option<BlockId>,
    ) -> Result<Bytes, Self::Error> {
        todo!("we should be able to call. We will have to consider adding a function to the `SimulationEnvironment` that uses `transact` and not `transact_commit`")
    }

    async fn get_logs(&self, _filter: &Filter) -> Result<Vec<Log>, Self::Error> {
        todo!("we should be able to get logs.")
    }

    async fn watch<'a>(
        &'a self,
        _filter: &Filter,
    ) -> Result<FilterWatcher<'a, Self::Provider, Log>, Self::Error> {
        todo!("we should be able to watch. we already have this partially implemented for agents.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_sender() {
        todo!("This should be an Agent's address")
    }

    async fn send_transaction() {
        todo!("Send shouldn't be called as we just need to FILL transactions.")
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
}
