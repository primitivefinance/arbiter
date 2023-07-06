#![allow(missing_docs)]
#![warn(unsafe_code)]

// TODO: 
// # Notes
// * The agent should be able to have multiple behaviors with different data types.
// * The agent should be able to be created before being attached to an environment.

use std::sync::Arc;
use revm::primitives::{Address, ExecutionResult, TxEnv};

use crate::environment::middleware::RevmMiddleware;

pub struct Agent {
    pub name: String,
    pub client: Arc<RevmMiddleware>,
    pub behaviors: Vec<Box<dyn Behavior<Data = dyn std::any::Any>>>,
}

impl Agent {
    pub fn new(name: String,
        tx_sender: crossbeam_channel::Sender<(TxEnv, crossbeam_channel::Sender<ExecutionResult>)>,
    ) -> Self {
        Self {
            name,
            client: Arc::new(RevmMiddleware::new(tx_sender)),
            behaviors: vec![],
        }
    }

    pub fn add_behavior<B>(&mut self, behavior: B)
    where
        B: Behavior<Data = dyn std::any::Any> + 'static,
    {
        self.behaviors.push(Box::new(behavior));
    }
}

use anyhow::Result;

pub trait Behavior {
    type Data;
    fn act(&self) -> Result<()>;
    fn condition(&self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBehavior {
        _data: String,
    }

    impl Behavior for TestBehavior {
        // TODO: We may not need the Data type inside of the trait.
        type Data = String;
        fn act(&self) -> Result<()> {
            Ok(())
        }
        fn condition(&self) -> Result<()> {
            Ok(())
        }
    }

    struct TestBehavior2 {
        _data: u64,
    }

    impl Behavior for TestBehavior2 {
        type Data = u64;
        fn act(&self) -> Result<()> {
            Ok(())
        }
        fn condition(&self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn add_behavior() {}

    #[test]
    fn multiple_behavior_data() {
        let mut _agent = Agent::new(crossbeam_channel::unbounded().0);
        // TODO: Do something like this to make sure this works.
        // agent.add_behavior(TestBehavior {
        //     data: "test".to_string(),
        // });
        // agent.add_behavior(TestBehavior2 { data: U256::zero() });
    }
}
