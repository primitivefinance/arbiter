#![allow(deprecated)]
#![warn(missing_docs)]
#![warn(unsafe_code)]

//! ## Architect
//!
//! Architect is the bundling, simulation and execution module of Arbiter.

use ethers::{
    core::{rand::thread_rng, types::transaction::eip2718::TypedTransaction},
    prelude::*,
    signers::Signer,
};
use ethers_flashbots::*;
use thiserror::Error;
use url::Url;

/// Type that represents an `Architect`, a transaction executor designed to
/// execute, simulate and bundle arbitrage opportunities.
#[allow(warnings)]
#[deprecated(since = "0.0.1", note = "will be useful for actors in the future")]
#[derive(Debug)]
pub struct Architect<S>
where
    S: Signer,
{
    /// Client that signs transactions.
    pub client: SignerMiddleware<FlashbotsMiddleware<Provider<Http>, LocalWallet>, S>,
    /// Bundle to be executed.
    pub bundle: BundleRequest,
}

/// Errors for bundle construction or execution.
#[derive(Error, Debug)]
#[deprecated(since = "0.0.1", note = "will be useful for actors in the future")]
#[allow(warnings)]
pub enum ArchitectError {
    /// Error with parsing the Flashbots relay URL.
    #[error(transparent)]
    RelayParseError(#[from] url::ParseError),

    /// Error with signing a transaction.
    #[error("an error occured when signing a bundle transaction")]
    SigningError,

    /// Error with fetching block number from middleware.
    #[error("an error occured when fetching the current block number")]
    BlockNumberError,
}

/// Type that represents an execution result from either a send or simulation.
#[deprecated(since = "0.0.1", note = "will be useful for actors in the future")]
#[allow(warnings)]
pub type ExecutionResult<T> = Result<T, FlashbotsMiddlewareError<Provider<Http>, LocalWallet>>;
#[deprecated(since = "0.0.1", note = "will be useful for actors in the future")]
#[allow(warnings)]
impl<S: Signer> Architect<S> {
    /// Public constructor function that instantiates an `Architect`.
    pub async fn new(provider: Provider<Http>, wallet: S) -> Result<Self, ArchitectError> {
        // This is your searcher identity.
        // It does not store funds and is not used for transaction execution.
        let bundle_signer = LocalWallet::new(&mut thread_rng());
        let bundle = BundleRequest::new();

        let relay = match Url::parse("https://relay.flashbots.net") {
            Err(err) => return Err(ArchitectError::RelayParseError(err)),
            Ok(url) => url,
        };

        let client = SignerMiddleware::new(
            FlashbotsMiddleware::new(provider, relay, bundle_signer),
            wallet,
        );

        let block_number = match client.get_block_number().await {
            Err(_) => return Err(ArchitectError::BlockNumberError),
            Ok(num) => num,
        };

        Ok(Self {
            client,
            bundle: bundle
                .set_block(block_number + 1)
                .set_simulation_block(block_number)
                .set_simulation_timestamp(0),
        })
    }

    /// Add and sign a transaction to the bundle to be executed.
    #[deprecated(since = "0.0.1", note = "will be useful for actors in the future")]
    #[allow(warnings)]
    pub async fn add_transactions(
        mut self,
        transactions: &Vec<TypedTransaction>,
    ) -> Result<Self, ArchitectError> {
        for tx in transactions {
            let signature = match self.client.signer().sign_transaction(tx).await {
                Err(_) => return Err(ArchitectError::SigningError),
                Ok(sig) => sig,
            };

            self.bundle = self.bundle.push_transaction(tx.rlp_signed(&signature));
        }

        Ok(self)
    }

    /// Simulate bundle execution.
    #[deprecated(since = "0.0.1", note = "will be useful for actors in the future")]
    #[allow(warnings)]
    pub async fn simulate(&mut self) -> ExecutionResult<SimulatedBundle> {
        self.client.inner().simulate_bundle(&self.bundle).await
    }

    /// Send the bundle.
    #[allow(warnings)]
    #[deprecated(since = "0.0.1", note = "will be useful for actors in the future")]
    pub async fn send(
        &mut self,
    ) -> ExecutionResult<
        PendingBundle<
            '_,
            <FlashbotsMiddleware<Provider<Http>, LocalWallet> as Middleware>::Provider,
        >,
    > {
        self.client.inner().send_bundle(&self.bundle).await
    }
}

#[cfg(test)]
mod tests {
    use ethers::{
        core::rand::thread_rng, prelude::*, types::transaction::eip2718::TypedTransaction,
    };

    use crate::Architect;

    // We will need more tests in future but this just ensures basic functionality is working.
    #[tokio::test]
    async fn test_architect_creation() {
        let provider = Provider::<Http>::try_from("https://mainnet.eth.aragon.network").unwrap();
        let tx = TypedTransaction::Legacy(TransactionRequest::pay("vitalik.eth", 100));

        let _architect = Architect::new(provider, LocalWallet::new(&mut thread_rng()))
            .await
            .unwrap()
            .add_transactions(&vec![tx])
            .await
            .unwrap();
    }
}
