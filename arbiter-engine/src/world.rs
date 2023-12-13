#![warn(missing_docs)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Probably should move labels to world instead of on the environment.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The world module contains the core world abstraction for the Arbiter Engine.

use crossbeam_channel::{Receiver, Sender};
use ethers::providers::{Provider, PubsubClient};

use super::*;
use crate::{agent::Agent, messager::Message};

/// A world is a collection of agents that use the same type of provider, e.g.,
/// operate on the same blockchain or same `Environment`.
pub struct World<P, E, A> {
    /// The identifier of the world.
    pub id: String,

    /// The agents in the world.
    pub agents: Vec<Agent<E, A>>, /* TODO: This should be a map of agents. Also, we should not
                                   * carry up these generics. */

    /// The provider for the world.
    pub provider: Provider<P>, /* TODO: The world itself may not need to carry around the
                                * provider, but the agents should all use the same type of
                                * provider. */

    /// The interconnects between different worlds.
    /// These can be used, for instance, to pass messages between agents running
    /// on different blockchain networks (e.g., Ethereum and Optimism).
    pub interconnects: HashMap<String, Interconnect>,
}

/// An interconnect is a connection between two worlds.
pub struct Interconnect {
    /// The message sender.
    _sender: Sender<Message>,

    /// The message receiver.
    _receiver: Receiver<Message>,
}

impl<P, E, A> World<P, E, A>
where
    P: PubsubClient,
    E: Send + Clone + 'static + std::fmt::Debug,
    A: Send + Clone + 'static + std::fmt::Debug,
{
    // TODO: May not need to take in the provider here, but rather get it from the
    // agents.
    /// Creates a new world with the given identifier and provider.
    pub fn new(id: &str, provider: Provider<P>) -> Self {
        Self {
            id: id.to_owned(),
            agents: vec![],
            provider,
            interconnects: HashMap::new(),
        }
    }

    /// Adds an agent to the world.
    pub fn add_agent(&mut self, agent: Agent<E, A>) {
        self.agents.push(agent);
    }

    /// Runs the agents in the world.
    pub async fn run(&mut self) {
        for agent in self.agents.iter_mut() {
            let mut joinset = agent.engine.take().unwrap().run().await.unwrap();
            while let Some(next) = joinset.join_next().await {
                if let Err(e) = next {
                    panic!("Error: {:?}", e);
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
    use crate::messager::Messager;

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
        agent.add_collector(messager);
        agent.add_executor(MempoolExecutor::new(client.clone()));
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
