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

use ethers::providers::{Provider, PubsubClient};

use super::*;
use crate::{agent::Agent, messager::Messager};

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
    pub messager: Messager,
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
    pub fn add_agent(&mut self, agent: Agent) {
        // TODO: Here is where we can maybe consider giving the agents a client?
        let id = agent.id.clone();
        self.agents.insert(id, agent);
    }

    /// Runs the agents in the world.
    pub async fn run(&mut self) {
        debug!("Running world: {}", self.id);

        for agent in self.agents.values_mut() {
            let join_sets = agent.run().await;
            for mut join_set in join_sets {
                debug!("Joining agent: {}", agent.id);
                while let Some(next) = join_set.join_next().await {
                    if let Err(e) = next {
                        panic!("Error: {:?}", e);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use arbiter_bindings::bindings::weth::WETH;
    use arbiter_core::{
        environment::builder::EnvironmentBuilder,
        middleware::{connection::Connection, RevmMiddleware},
    };
    use artemis_core::executors::mempool_executor::MempoolExecutor;
    use ethers::{
        providers::{Middleware, Provider, Ws},
        types::Address,
    };
    use futures_util::StreamExt;

    use super::*;
    use crate::{agent::BehaviorBuilder, messager::Messager};

    #[ignore]
    #[test]
    fn arbiter_world() {
        let environment = EnvironmentBuilder::new().build();
        let connection = Connection::from(&environment);
        let provider = Provider::new(connection);
        let mut world = World::new("test_world", provider);

        let client = RevmMiddleware::new(&environment, Some("testname")).unwrap();
        let mut agent = Agent::new("agent1");
        let messager = Messager::new();
        let behavior = BehaviorBuilder::new()
            .add_collector(messager.clone())
            .add_executor(MempoolExecutor::new(client.clone()))
            .build();
        world.add_agent(agent);
    }

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
