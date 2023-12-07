// NOTES: Each agent essentially has its own engine. We can collect all of the
// engines together into a world.

// CAN GIVE AGENT A CALCULATOR EVM TOO!

// TODO: We may need traits for Events and Actions (e.g., "Event" and "Action"
// which have a method like "parse()" and "produce()" or something.). TODO: Need
// an init signal or something.

use artemis_core::{
    engine::Engine,
    types::{Collector, Executor},
};

pub struct Agent<E, A> {
    id: String, // TODO: We might not really need an ID here specifically. Hard to say right now.
    engine: Engine<E, A>, // Note, agent shouldn't need a client as the engine will handle this.
    dependencies: Vec<String>,
    dependents: Vec<String>,
}

impl<E, A> Agent<E, A>
where
    E: Send + Clone + 'static + std::fmt::Debug,
    A: Send + Clone + 'static + std::fmt::Debug,
{
    #[allow(clippy::new_without_default)]
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            engine: Engine::new(),
            dependencies: vec![],
            dependents: vec![],
        }
    }

    pub fn add_collector(&mut self, collector: impl Collector<E> + 'static) {
        self.engine.add_collector(Box::new(collector));
    }

    pub fn add_executor(&mut self, executor: impl Executor<A> + 'static) {
        self.engine.add_executor(Box::new(executor));
    }

    pub fn add_dependency(&mut self, dependency: &str) {
        self.dependencies.push(dependency.to_owned());
    }

    pub fn add_dependent(&mut self, dependent: &str) {
        self.dependents.push(dependent.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
    use arbiter_core::{environment::builder::EnvironmentBuilder, middleware::RevmMiddleware};
    use artemis_core::{
        collectors::log_collector::LogCollector,
        executors::mempool_executor::{MempoolExecutor, SubmitTxToMempool},
    };
    use ethers::types::U256;

    use super::*;

    #[tokio::test]
    async fn test_agent() {
        // Startup
        let environment = EnvironmentBuilder::new().build();
        let client = RevmMiddleware::new(&environment, None).unwrap();
        let arb = ArbiterToken::deploy(
            client.clone(),
            ("Arbiter Token".to_string(), "ARB".to_string(), 18),
        )
        .unwrap()
        .send()
        .await
        .unwrap();

        // Build the agent
        let mut agent = Agent::new("test");
        let collector = LogCollector::new(client.clone(), arb.transfer_filter().filter);
        agent.add_collector(collector);
        let executor = MempoolExecutor::new(client.clone());
        agent.add_executor(executor);

        let tx = arb.mint(client.address(), U256::from(1)).tx;
        let _submit_tx = SubmitTxToMempool {
            tx,
            gas_bid_info: None,
        };

        // TODO: We should write a test that runs the agent's engine in some
        // meaningful way.
    }
}
