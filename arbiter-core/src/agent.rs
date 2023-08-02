#![allow(missing_docs)]
#![warn(unsafe_code)]

use std::sync::Arc;

use ethers::providers::Middleware;

use crate::middleware::RevmMiddleware;

pub trait Attached {
    type Client;
}
pub struct IsAttached<M: Middleware> {
    marker: std::marker::PhantomData<M>,
}
pub struct NotAttached {}
impl<M: Middleware> Attached for IsAttached<M> {
    type Client = Arc<M>;
}
impl Attached for NotAttached {
    type Client = ();
}

pub struct Agent<A: Attached> {
    pub name: String,
    pub client: A::Client,
    pub behaviors: Vec<Box<dyn Behavior>>,
}


impl Agent<NotAttached> {
    pub(crate) fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            client: (),
            behaviors: vec![],
        }
    }

    pub fn add_behavior<B>(&mut self, behavior: B)
    where
        B: Behavior + 'static,
    {
        self.behaviors.push(Box::new(behavior));
    }

    pub fn attach_to_client<M: Middleware>(self, client: Arc<M>) -> Agent<IsAttached<M>> {
        Agent::<IsAttached<M>> {
            name: self.name,
            client,
            behaviors: self.behaviors,
        }
    }

    pub fn attach_to_environment(self, environment: &mut crate::environment::Environment) {
        let middleware = RevmMiddleware::new(&self, environment);
        let agent_attached = self.attach_to_client(middleware.into());
        environment.agents.push(agent_attached);
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

    use ethers::providers::{MockProvider, ProviderError};

    use super::*;

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
        let mut agent = Agent::new(name);

        // Add a behavior of the first type.
        let data = TEST_BEHAVIOR_DATA.to_string();
        let behavior = TestBehavior { data };
        agent.add_behavior(behavior);
        assert!(agent.behaviors.len() == 1);
        assert!(agent.behaviors[0].process_event().await);
        agent.behaviors[0].sync_state();
    }
}
