#![allow(missing_docs)]
#![warn(unsafe_code)]

use std::sync::Arc;

use ethers::providers::Middleware;

use crate::{environment::Connection, middleware::RevmMiddleware};

pub struct Agent<M: Middleware> {
    pub name: String,
    pub client: Arc<M>,
    pub behaviors: Vec<Box<dyn Behavior>>,
}

impl<M: Middleware> std::fmt::Debug for Agent<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Agent")
            .field("name", &self.name)
            .field("client", &self.client)
            .finish()
    }
}

impl Agent<RevmMiddleware> {
    pub fn new_simulation_agent(name: String, connection: Connection) -> Self {
        Self {
            name,
            client: Arc::new(RevmMiddleware::new(connection)),
            behaviors: vec![],
        }
    }
}

impl<M: Middleware> Agent<M> {
    pub fn new(name: String, middleware: M) -> Self {
        Self {
            name,
            client: Arc::new(middleware),
            behaviors: vec![],
        }
    }

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
pub(crate) mod tests {
    pub(crate) const TEST_AGENT_NAME: &str = "test_agent";
    pub(crate) const TEST_BEHAVIOR_DATA: &str = "test_behavior_data";

    use super::*;
    use ethers::providers::{MockProvider, ProviderError};

    #[derive(Debug)]
    pub(crate) struct TestMiddleware {}

    impl Middleware for TestMiddleware {
        type Inner = Self;
        type Provider = MockProvider;
        type Error = ProviderError;

        fn inner(&self) -> &Self::Inner {
            self
        }
    }

    pub(crate) struct TestBehavior {
        data: String,
    }

    #[async_trait::async_trait]
    impl Behavior for TestBehavior {
        async fn process_event(&mut self) -> bool {
            true
        }
        fn sync_state(&mut self) {
            assert_eq!(self.data, TEST_BEHAVIOR_DATA.to_string());
        }
    }

    #[tokio::test]
    async fn agent_behavior() {
        let name = TEST_AGENT_NAME.to_string();
        let mut agent = Agent::new(name, TestMiddleware {});

        // Add a behavior of the first type.
        let data = TEST_BEHAVIOR_DATA.to_string();
        let behavior = TestBehavior { data };
        agent.add_behavior(behavior);
        assert!(agent.behaviors.len() == 1);
        assert!(agent.behaviors[0].process_event().await);
        agent.behaviors[0].sync_state();
    }
}
