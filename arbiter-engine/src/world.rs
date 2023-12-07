// TODO: Probalby should move labels to world instead of environment.

use crate::agent::Agent;

// In order to add dependencies and what not, we need all the agents to be in the world or something probably.
use super::*;

use ethers::providers::{Provider, PubsubClient};

pub struct World<P, E, A> {
    pub agents: Vec<Agent<E, A>>,
    pub provider: Provider<P>,
    // pub interconnects: Vec<World>,
}

impl<P, E, A> World<P, E, A>
where
    P: PubsubClient,
{
    pub fn new(provider: Provider<P>) -> Self {
        Self {
            agents: vec![],
            provider,
        }
    }

    pub fn add_agent(&mut self, agent: Agent<E, A>) {
        self.agents.push(agent);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arbiter_core::{
        environment::builder::EnvironmentBuilder, middleware::connection::Connection,
    };

    use ethers::{
        providers::{Middleware, Provider, Ws},
        types::Address,
    };
    use std::str::FromStr;

    use arbiter_bindings::bindings::weth::WETH;
    use futures_util::StreamExt;

    #[test]
    fn arbiter_world() {
        let environment = EnvironmentBuilder::new().build();
        let connection = Connection::from(&environment);
        let provider = Provider::new(connection);
        // let mut world = World::new(provider);

        // let client = RevmMiddleware::new(connection, Some("testname"));
        // let agent = Agent::new("agent1");
        // world.add_agent(agent);
    }

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
