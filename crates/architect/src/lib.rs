#![warn(missing_docs)]
#![forbid(unsafe_code)]

//! ## Architect
//!
//! Architect is the bundling, simulation and execution module of Arbiter.

use ethers::core::rand::thread_rng;
use ethers::prelude::*;
use ethers::signers::Signer;
use ethers_flashbots::*;
use ethers::core::types::transaction::eip2718::TypedTransaction;
use url::Url;

/// Type that represents an `Architect`, a transaction executor designed to
/// execute, simulate and bundle arbitrage opportunities.
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

/// Type that represents an execution result from either a send or simulation.
pub type ExecutionResult<T> = Result<T, FlashbotsMiddlewareError<Provider<Http>, LocalWallet>>;

impl<S: Signer> Architect<S> {
    /// Public constructor function that instantiates an `Architect`.
    pub async fn new(provider: Provider<Http>, wallet: S) -> Self {
        // This is your searcher identity.
        // It does not store funds and is not used for transaction execution.
        let bundle_signer = LocalWallet::new(&mut thread_rng());
        let bundle = BundleRequest::new();
        let client = SignerMiddleware::new(
            FlashbotsMiddleware::new(
                provider,
                Url::parse("https://relay.flashbots.net").unwrap(),
                bundle_signer,
            ),
            wallet,
        );

        let block_number = client.get_block_number().await.unwrap();

        Self { 
            client, 
            bundle: bundle.set_block(block_number + 1).set_simulation_block(block_number),
        }
    }

    /// Add and sign a transaction to the bundle to be executed.
    pub async fn add_transaction(self, transaction: TypedTransaction) {
        let signature = self.client.signer().sign_transaction(&transaction).await.unwrap();
        self.bundle.push_transaction(transaction.rlp_signed(&signature));
    }

    /// Simulate bundle execution.
    pub async fn simulate(&mut self) -> ExecutionResult<SimulatedBundle> {
        self.client.inner().simulate_bundle(&self.bundle).await
    }

    /// Send the bundle.
    pub async fn send(&mut self) -> ExecutionResult<PendingBundle<'_, <FlashbotsMiddleware<Provider<Http>, LocalWallet> as Middleware>::Provider>> {
        self.client.inner().send_bundle(&self.bundle).await
    }
}

#[cfg(test)]
mod tests {
    use crate::Architect;
    use ethers::prelude::*;
    use ethers::core::rand::thread_rng;

    // We will need more tests in future but this just ensures basic functionality is working.
    #[tokio::test]
    async fn test_architect_creation() {
        let provider = Provider::<Http>::try_from("https://mainnet.eth.aragon.network").unwrap();
        let wallet = LocalWallet::new(&mut thread_rng());

        let _architect = Architect::new(provider, wallet).await;
    }
}