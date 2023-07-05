#![warn(missing_docs)]
#![warn(unsafe_code)]

//! ## Agent trait and associated functionality
//!
//! An abstract representation of an agent on the EVM, to be used in simulations.
//! Some examples of agents are market makers or arbitrageurs.
//! All agents must implement the [`Agent`] traits and be included in the [`AgentType`] enum.
//!

use crossbeam_channel::Sender;
use ethers::{
    prelude::k256::ecdsa::{
        signature::hazmat::PrehashSigner, RecoveryId, Signature as RecoverySignature,
    },
    providers::Middleware,
    signers::{LocalWallet, Wallet},
};
use rand::thread_rng;
use std::sync::Arc;
use thiserror::Error;

use crate::environment::{middleware::RevmMiddleware, SimulationEnvironment};
use ethers::providers::{MockProvider, Provider};
use revm::primitives::{Address, ExecutionResult, TxEnv};
// pub type Signature = ecdsa_core::Signature<Secp256k1>;

pub struct Agent {
    pub client: Arc<RevmMiddleware>,
    pub behaviors: Vec<Box<dyn Behavior<Data = dyn std::any::Any>>>,
}

impl Agent {
    pub fn new(
        tx_sender: crossbeam_channel::Sender<(TxEnv, crossbeam_channel::Sender<ExecutionResult>)>,
    ) -> Self {
        Self {
            client: Arc::new(RevmMiddleware::new(tx_sender)),
            behaviors: vec![],
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
    type Data;
    fn act(&self) -> Result<()>;
    fn watch(&self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::U256;

    struct TestBehavior {
        data: String,
    }

    impl Behavior for TestBehavior {
        type Data = String;
        fn act(&self) -> Result<()> {
            Ok(())
        }
        fn watch(&self) -> Result<()> {
            Ok(())
        }
    }

    struct TestBehavior2 {
        data: U256,
    }

    impl Behavior for TestBehavior2 {
        type Data = U256;
        fn act(&self) -> Result<()> {
            Ok(())
        }
        fn watch(&self) -> Result<()> {
            Ok(())
        }
    }

    fn multiple_behavior_data() {
        let mut agent = Agent::new(crossbeam_channel::unbounded().0);
        // TODO: Do something like this to make sure this works.
        // agent.add_behavior(TestBehavior {
        //     data: "test".to_string(),
        // });
        // agent.add_behavior(TestBehavior2 { data: U256::zero() });
    }
}
