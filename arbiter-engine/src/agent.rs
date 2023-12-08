#![warn(missing_docs)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// We may need traits for Events and Actions (e.g., "Event" and "Action"
// which have a method like "parse()" and "produce()" or something.).
// Need an init signal or something.
// We can give agents a "calculator" evm to send "Actions" to when they are just
// doing compute so they aren't blocking the main tx thread.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The agent module contains the core agent abstraction for the Arbiter Engine.

use artemis_core::{
    engine::Engine,
    types::{Collector, Executor, Strategy},
};

/// An agent is an entity capable of processing events and producing actions.
/// These are the core actors in simulations or in onchain systems.
/// Agents can be connected of other agents either as a dependent, or a
/// dependency.
pub struct Agent<E, A> {
    /// Identifier for this agent.
    /// Used for routing messages.
    pub id: String,

    /// The engine that this agent uses to process events and produce actions.
    pub(crate) engine: Option<Engine<E, A>>, /* Note, agent shouldn't NEED a client as a field as the engine can
                                              * handle this. */

    /// Agents that this agent depends on.
    pub dependencies: Vec<String>,

    /// Agents that depend on this agent.
    pub dependents: Vec<String>,
}

impl<E, A> Agent<E, A>
where
    E: Send + Clone + 'static + std::fmt::Debug,
    A: Send + Clone + 'static + std::fmt::Debug,
{
    #[allow(clippy::new_without_default)]
    /// Produces a new agent with the given identifier.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            engine: Some(Engine::new()),
            dependencies: vec![],
            dependents: vec![],
        }
    }

    /// Adds a collector to the agent's engine.
    pub fn add_collector(&mut self, collector: impl Collector<E> + 'static) {
        self.engine
            .as_mut()
            .expect("Engine has already been taken by the `World::run()` method.")
            .add_collector(Box::new(collector));
    }

    /// Adds an executor to the agent's engine.
    pub fn add_executor(&mut self, executor: impl Executor<A> + 'static) {
        self.engine
            .as_mut()
            .expect("Engine has already been taken by the `World::run()` method.")
            .add_executor(Box::new(executor));
    }

    /// Adds a strategy to the agent's engine.
    pub fn add_strategy(&mut self, strategy: impl Strategy<E, A> + 'static) {
        self.engine
            .as_mut()
            .expect("Engine has already been taken by the `World::run()` method.")
            .add_strategy(Box::new(strategy));
    }

    /// Adds a dependency to the agent.
    /// Dependencies are agents that this agent depends on.
    pub fn add_dependency(&mut self, dependency: &str) {
        self.dependencies.push(dependency.to_owned());
    }

    /// Adds a dependent to the agent.
    /// Dependents are agents that depend on this agent.
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
    }
}
