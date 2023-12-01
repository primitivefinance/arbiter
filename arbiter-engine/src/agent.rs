use std::sync::{Arc, Weak};

use ethers::{contract::Contract, middleware::Middleware};

pub trait Behavior {
    fn startup(&mut self);
    fn process_event(&mut self);
    fn act(&mut self);
}

pub struct Agent<M: Middleware> {
    client: Arc<M>,
    behavior: Option<Arc<dyn Behavior>>,
    // contracts: Vec<Contract>,
    dependencies: Vec<Weak<Agent<M>>>,
    dependents: Vec<Arc<Agent<M>>>,
}

impl<M: Middleware> Agent<M> {
    pub fn new(client: Arc<M>) -> Self {
        Self {
            client,
            behavior: None,
            dependencies: vec![],
            dependents: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use arbiter_core::{environment::builder::EnvironmentBuilder, middleware::RevmMiddleware};

    struct TestBehavior;

    impl Behavior for TestBehavior {
        fn startup(&mut self) {
            todo!()
        }

        fn process_event(&mut self) {
            todo!()
        }

        fn act(&mut self) {
            todo!()
        }
    }

    #[test]
    fn test_agent() {
        let environment = EnvironmentBuilder::new().build();
        let client = RevmMiddleware::new(&environment, None).unwrap();
        let behavior = Arc::new(TestBehavior);
        let agent = Agent::new(client);
    }
}
