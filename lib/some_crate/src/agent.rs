#![allow(missing_docs)]
#![warn(unsafe_code)]

// TODO:
// # Notes
// * The agent should be able to have multiple behaviors with different data types.
// * The agent should be able to be created before being attached to an environment.

use std::sync::Arc;

use crate::{environment::RevmEnvironment, middleware::RevmMiddleware};

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
    fn new(name: String, environment: RevmEnvironment) -> Self {
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
    const TEST_ENV_LABEL: &str = "test_env";
    const TEST_AGENT_NAME: &str = "test_agent";
    const TEST_BEHAVIOR_DATA: &str = "test_behavior_data";
    use super::*;

    struct TestBehavior {
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
    async fn test_behavior() {
        let label = TEST_ENV_LABEL.to_string();
        let name = TEST_AGENT_NAME.to_string();
        let mut agent = Agent::new(
            name,
            RevmEnvironment::new(label),
        );

        // Add a behavior of the first type.
        let data = TEST_BEHAVIOR_DATA.to_string();
        let behavior = TestBehavior {
            data,
        };
        agent.add_behavior(behavior);
        assert!(agent.behaviors.len() == 1);
        assert!(agent.behaviors[0].process_event().await);
        agent.behaviors[0].sync_state();
    }
}
