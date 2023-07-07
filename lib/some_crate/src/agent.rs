#![allow(missing_docs)]
#![warn(unsafe_code)]

// TODO:
// # Notes
// * The agent should be able to have multiple behaviors with different data types.
// * The agent should be able to be created before being attached to an environment.

use revm::primitives::{Address, ExecutionResult, TxEnv};
use std::sync::Arc;
use thiserror::Error;

use crate::{middleware::RevmMiddleware, environment::RevmEnvironment};

pub struct Agent {
    pub name: String,
    pub client: Arc<RevmMiddleware>,
    pub behaviors: Vec<Box<dyn Behavior>>,
}

impl std::fmt::Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Agent")
            .field("name", &self.name)
            .field("client", &self.client)
            .finish()
    }
}


impl Agent {
    fn new(
        name: String,
        environment: RevmEnvironment,
    ) -> Self {
        Self {
            name,
            client: Arc::new(RevmMiddleware::new(environment)),
            behaviors: vec![],
        }
    }
    // TODO: We probably don't need static lifetimes. We could just set the lifetime of the environment and everything inside of it. 
    // The manager can outlive any environment.
    pub fn add_behavior<B>(&mut self, behavior: B)
    where
        B: Behavior + 'static,
    {
        self.behaviors.push(Box::new(behavior));
    }
}

// TODO: Note -- Artemis uses a `process_event` function that returns an `Option<Action>` for something to happen. 
// https://github.com/paradigmxyz/artemis/blob/c8ab223a363a875f685ab177839eacfffc9d8de0/crates/artemis-core/src/types.rs#L25
#[async_trait::async_trait]
pub trait Behavior: Send + Sync {
    async fn process_event(&mut self) -> bool;
    fn sync_state(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBehavior {
        _data: String,
    }

    #[async_trait::async_trait]
    impl Behavior for TestBehavior {
        async fn process_event(&mut self) -> bool {
            true

        }
        fn sync_state(&mut self) {
            println!("Hello, world!")
        }
    }

    struct TestBehavior2 {
        _data: u64,
    }

    #[async_trait::async_trait]
    impl Behavior for TestBehavior2 {
        async fn process_event(&mut self) -> bool {
            false
        }
        fn sync_state(&mut self) {
            println!("Hello, world, it's me, number 2")
        }
    }

    #[test]
    fn add_behavior() {}

    // #[test]
    // fn multiple_behavior_data() {
    //     let mut _agent = Agent::new(crossbeam_channel::unbounded().0);
    //     // TODO: Do something like this to make sure this works.
    //     // agent.add_behavior(TestBehavior {
    //     //     data: "test".to_string(),
    //     // });
    //     // agent.add_behavior(TestBehavior2 { data: U256::zero() });
    // }
}
