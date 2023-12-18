#![warn(missing_docs)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// We may need traits for Events and Actions (e.g., "Event" and "Action"
// which have a method like "parse()" and "produce()" or something.).
// Need an init signal or something.
// We can give agents a "calculator" evm to send "Actions" to when they are just
// doing compute so they aren't blocking the main tx thread.
// Maybe by default we should give agents a messager as part of their engine so we can call a
// "start" and "stop" with them.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The agent module contains the core agent abstraction for the Arbiter Engine.

use std::{error::Error, pin::Pin};

use artemis_core::{
    engine::Engine,
    types::{Collector, Executor, Strategy},
};
use futures_util::Future;
use tokio::task::JoinSet;

/// An agent is an entity capable of processing events and producing actions.
/// These are the core actors in simulations or in onchain systems.
/// Agents can be connected of other agents either as a dependent, or a
/// dependency.
pub struct Agent {
    /// Identifier for this agent.
    /// Used for routing messages.
    pub id: String,

    /// The engine that this agent uses to process events and produce actions.
    pub(crate) behaviors: Vec<Behavior>, /* Note, agent shouldn't NEED a client as a field
                                          * as the engine can
                                          * handle this. */

    /// Agents that this agent depends on.
    pub dependencies: Vec<String>,

    /// Agents that depend on this agent.
    pub dependents: Vec<String>,
}

impl Agent {
    #[allow(clippy::new_without_default)]
    /// Produces a new agent with the given identifier.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            behaviors: vec![],
            dependencies: vec![],
            dependents: vec![],
        }
    }

    /// Adds a strategy to the agent's engine.
    /// Does so by pushing a future onto the agent's behavior vector.
    /// This future returns the `JoinSet<()>` of the engine.
    pub fn add_behavior<E, A>(&mut self, engine: Engine<E, A>)
    where
        E: Send + Clone + 'static + std::fmt::Debug,
        A: Send + Clone + 'static + std::fmt::Debug,
    {
        let fut = engine.run();
        self.behaviors.push(Box::pin(fut));
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

    pub(crate) async fn run(&mut self) -> Vec<JoinSet<()>> {
        let mut join_sets = vec![];
        for behavior in self.behaviors.iter_mut() {
            let joinset = behavior.await.unwrap();
            join_sets.push(joinset);
        }
        join_sets
    }
}

type Behavior = Pin<Box<dyn Future<Output = Result<JoinSet<()>, Box<dyn Error>>> + Send>>;

/// A behavior builder is used to build an agent's behavior.
/// A behavior is meant to be quite simple, and is composed of a collector, an
/// executor, and a strategy that uses those two components.
/// Agents can have multiple simple behaviors to allow for the agent to have an emergent complex set of actions.
pub struct BehaviorBuilder<E, A> {
    collectors: Vec<Box<dyn Collector<E>>>,
    executors: Vec<Box<dyn Executor<A>>>,
    strategies: Vec<Box<dyn Strategy<E, A>>>,
}

impl<E, A> BehaviorBuilder<E, A>
where
    E: Send + Clone + 'static + std::fmt::Debug,
    A: Send + Clone + 'static + std::fmt::Debug,
{
    /// Creates a new behavior builder.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            collectors: vec![],
            executors: vec![],
            strategies: vec![],
        }
    }

    /// Adds a collector to the behavior.
    pub fn add_collector(mut self, collector: impl Collector<E> + 'static) -> Self {
        self.collectors.push(Box::new(collector));
        self
    }

    /// Adds an executor to the behavior.
    pub fn add_executor(mut self, executor: impl Executor<A> + 'static) -> Self {
        self.executors.push(Box::new(executor));
        self
    }

    /// Adds a strategy to the behavior.
    pub fn add_strategy(mut self, strategy: impl Strategy<E, A> + 'static) -> Self {
        self.strategies.push(Box::new(strategy));
        self
    }

    /// Builds the behavior.
    pub fn build(self) -> Engine<E, A> {
        let mut engine = Engine::new();
        for collector in self.collectors {
            engine.add_collector(collector);
        }
        for executor in self.executors {
            engine.add_executor(executor);
        }
        for strategy in self.strategies {
            engine.add_strategy(strategy);
        }
        engine
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

    #[ignore]
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
        let behavior = BehaviorBuilder::new()
            .add_collector(LogCollector::new(
                client.clone(),
                arb.transfer_filter().filter,
            ))
            .add_executor(MempoolExecutor::new(client.clone()))
            .build();
        agent.add_behavior(behavior);

        let tx = arb.mint(client.address(), U256::from(1)).tx;
        let _submit_tx = SubmitTxToMempool {
            tx,
            gas_bid_info: None,
        };
    }
}
