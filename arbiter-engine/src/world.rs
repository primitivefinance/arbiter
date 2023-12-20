#![warn(missing_docs)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Probably should move labels to world instead of on the environment.
// One thing that is different about the Arbiter world is that give a bunch of different channels to
// communicate with the Environment's tx thread. This is different from a connection to a blockchain
// where you typically will just have a single HTTP/WS connection. What we want is some kind of way
// of having the world own a reference to a provider or something
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The world module contains the core world abstraction for the Arbiter Engine.

use artemis_core::types::{Collector, CollectorStream, Executor};
use ethers::{
    abi::Hash,
    providers::{Provider, PubsubClient},
};

use super::*;
use crate::{
    agent::Agent,
    messager::{Message, Messager},
};

/// A world is a collection of agents that use the same type of provider, e.g.,
/// operate on the same blockchain or same `Environment`.
pub struct World<P> {
    /// The identifier of the world.
    pub id: String,

    /// The agents in the world.
    pub agents: HashMap<String, Agent>, /* TODO: This should be a map of agents. We
                                         * may also want to add a bit more to the
                                         * Entity trait (e.g., the id of the agent).
                                         * In which case, we could expose it as pub so
                                         * those methods can be grabbed. */

    /// The provider for the world.
    pub provider: Provider<P>, /* TODO: The world itself may not need to carry around the
                                * provider, but the agents should all use the same type of
                                * provider. */

    /// The messaging layer for the world.
    pub messager: Messager, // TODO: Use this as the message executor that can be given to all agents and give each agent their specific collector.
}

// TODO: Can add a messager as an interconnect and have the manager give each
// world it owns a clone of the same messager.

impl<P> World<P>
where
    P: PubsubClient,
{
    // TODO: May not need to take in the provider here, but rather get it from the
    // agents.
    /// Creates a new world with the given identifier and provider.
    pub fn new(id: &str, provider: Provider<P>) -> Self {
        Self {
            id: id.to_owned(),
            agents: HashMap::new(),
            provider,
            messager: Messager::new(),
        }
    }

    /// Adds an agent to the world.
    pub fn create_agent(&mut self, id: &str) -> &mut Agent {
        // TODO: Here is where we can maybe consider giving the agents a client?
        let agent = Agent::connect(id, self);
        self.agents.insert(id.to_owned(), agent);
        self.agents.get_mut(id).unwrap()
    }

    /// Runs the agents in the world.
    pub async fn run(&mut self) -> Vec<tokio::task::JoinHandle<()>> {
        debug!("Running world: {}", self.id);
        debug!("Agents in world: {:?}", self.agents.keys());

        let mut tasks = Vec::new();

        for agent in self.agents.values_mut() {
            trace!("Running agent: {}", agent.id);
            let join_sets = Box::leak(Box::new(agent.run().await));
            for set in join_sets.iter_mut() {
                let task = tokio::spawn(async move {
                    while let Some(next) = set.join_next().await {
                        if let Err(e) = next {
                            panic!("Error: {:?}", e);
                        }
                    }
                });
                tasks.push(task);
            }
        }
        tasks
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use arbiter_bindings::bindings::weth::WETH;
    use arbiter_core::{
        environment::builder::EnvironmentBuilder, middleware::connection::Connection,
    };
    use ethers::{
        providers::{Middleware, Provider, Ws},
        types::Address,
    };
    use futures_util::StreamExt;

    use super::*;

    // #[ignore]
    // #[test]
    // fn arbiter_world() {
    //     let environment = EnvironmentBuilder::new().build();
    //     let connection = Connection::from(&environment);
    //     let provider = Provider::new(connection);
    //     let mut world = World::new("test_world", provider);

    //     // let client = RevmMiddleware::new(&environment, Some("testname")).unwrap();
    //     let agent = Agent::new("agent1");
    //     // let messager = Messager::new();
    //     // let behavior = BehaviorBuilder::new()
    //     //     .add_collector(messager.clone())
    //     //     .add_executor(MempoolExecutor::new(client.clone()))
    //     //     .build();
    //     world.add_agent(agent);
    // }

    #[ignore]
    #[tokio::test]
    async fn mainnet_world() {
        let ws_url = std::env::var("MAINNET_WS_URL").expect("MAINNET_WS_URL must be set");
        let ws = Ws::connect(ws_url).await.unwrap();
        let provider = Provider::new(ws);
        // let mut world = World::new(provider);
        let client = Arc::new(provider);
        let weth = WETH::new(
            Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap(),
            client.clone(),
        );
        let filter = weth.approval_filter().filter;
        let mut subscription = client.subscribe_logs(&filter).await.unwrap();
        println!("next: {:?}", subscription.next().await);
    }
}
