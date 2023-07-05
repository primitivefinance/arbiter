#![warn(missing_docs)]
#![warn(unsafe_code)]

//! ## Agent trait and associated functionality
//!
//! An abstract representation of an agent on the EVM, to be used in simulations.
//! Some examples of agents are market makers or arbitrageurs.
//! All agents must implement the [`Agent`] traits and be included in the [`AgentType`] enum.
//! 


use std::sync::Arc;
use thiserror::Error;
use ethers::{signers::{Wallet, Signer}, providers::Middleware};
use crossbeam_channel::Sender;

use ethers::providers::{Provider, MockProvider};
use revm::primitives::{
    Address, ExecutionResult, TxEnv,
};
use crate::environment::SimulationEnvironment;

// can an agent be be a struct? well API wants users
#[derive(Debug)]
pub struct AgentMiddleware<S> where S: Signer {
    /// tansaction sender
    pub tx_sender: Sender<(TxEnv, Sender<ExecutionResult>)>,
    // Provider
    pub provider: Provider<MockProvider>,
    // Wallet
    pub wallet: Wallet<S>,

}

pub(crate) struct Agent<S, D, B> where S: Signer, B: Behavior {
    pub client: Arc<AgentMiddleware<S>>,
    pub data: D,
    pub behaviors: Vec<B>,
}


impl<S, D, B> Agent<S, D, B> {
    pub fn new(env: SimulationEnvironment) -> Self{
        Self {
            client: Arc::new(AgentMiddleware::new(env, Wallet::new(&mut rand::thread_rng()))),
            data: todo!(),
            behaviors: vec![],
        }
    }
}


impl<S> AgentMiddleware<S> {
    pub fn new(env: SimulationEnvironment, wallet: Wallet<S>) -> Self {
        Self {
            tx_sender: env.transaction_channel.0,
            provider: env.provider,
            wallet,
        }
    }
}


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
pub trait AgentProperties {
    // type Data;
    // type FilterWatcher: Stream<Item = Result<(Vec<Token>, usize), AbiError>> + Send + Sync;
    /// Returns the address of the agent.
    fn address(&self) -> Address {
        self.address()
    }
    /// The event's channel receiver for the agent.

    async fn act(&self) -> Result<()> {
        Ok(())
    }

    fn name(&self) -> String {
        "Agent".to_string()
    }

}
pub trait Behavior {
    fn act(&self) -> Result<()>;
    fn watch(&self) -> Result<()>;
}


#[cfg(test)]
mod tests {}
