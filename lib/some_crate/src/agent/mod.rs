#![warn(missing_docs)]
#![warn(unsafe_code)]

//! ## Agent trait and associated functionality
//!
//! An abstract representation of an agent on the EVM, to be used in simulations.
//! Some examples of agents are market makers or arbitrageurs.
//! All agents must implement the [`Agent`] traits and be included in the [`AgentType`] enum.
//! 


use ethers::{signers::{Wallet, Signer}, providers::Middleware};
use crossbeam_channel::Sender;

use ethers::providers::{Provider, MockProvider};
use revm::primitives::{
    Address, ExecutionResult, TxEnv,
};
use crate::environment::SimulationEnvironment;

// can an agent be be a struct? well API wants users
#[derive(Debug)]
pub struct AgentMiddleware {
    /// The address of the agent.
    pub address: Address,
    /// tansaction sender
    pub tx_sender: Sender<(TxEnv, Sender<ExecutionResult>)>,
    // Provider
    pub provider: Provider<MockProvider>,

}

impl AgentMiddleware {
    pub fn address(&self) -> Address {
        self.address
    }
    pub fn new(env: SimulationEnvironment) -> Self {
        Self {
            address: Wallet::new(&mut rand::thread_rng()).address().into(),
            tx_sender: env.transaction_channel.0,
            provider: env.provider,
        }
    }
}


use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("This is and error")]
    SomeError,
    #[error("This is another error")]
    SomeOtherError,
}

use anyhow::Result;

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM.
#[async_trait::async_trait]
pub trait Agent {
    // type FilterWatcher: Stream<Item = Result<(Vec<Token>, usize), AbiError>> + Send + Sync;
    /// Returns the address of the agent.
    fn address(&self) -> Address;
    /// The event's channel receiver for the agent.

    async fn act(&self) -> Result<()> {
        Ok(())
    }

    fn name(&self) -> String {
        "Agent".to_string()
    }

}


#[cfg(test)]
mod tests {}
