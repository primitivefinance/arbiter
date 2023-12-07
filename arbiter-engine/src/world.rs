// TODO: Probalby should move labels to world instead of environment.

use crate::{agent::Agent, messager::Message};

// In order to add dependencies and what not, we need all the agents to be in the world or something probably.
use super::*;

use crossbeam_channel::{Receiver, Sender};
use ethers::providers::{Provider, PubsubClient};

pub struct World<P, E, A> {
    pub id: String,
    pub agents: Vec<Agent<E, A>>,
    pub provider: Provider<P>,
    pub interconnects: HashMap<String, Interconnect>,
}

pub struct Interconnect {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl<P, E, A> World<P, E, A>
where
    P: PubsubClient,
{
    pub fn new(id: &str, provider: Provider<P>) -> Self {
        Self {
            id: id.to_owned(),
            agents: vec![],
            provider,
            interconnects: HashMap::new(),
        }
    }

    pub fn add_agent(&mut self, agent: Agent<E, A>) {
        self.agents.push(agent);
    }
}

#[cfg(test)]
mod tests {
    use crate::messager::Messager;

    use super::*;
    use arbiter_core::{
        environment::builder::EnvironmentBuilder,
        middleware::{connection::Connection, RevmMiddleware},
    };

    use artemis_core::executors::mempool_executor::MempoolExecutor;
    use ethers::{
        providers::{Middleware, Provider, Ws},
        types::Address,
    };
    use std::str::FromStr;

    use arbiter_bindings::bindings::weth::WETH;
    use futures_util::StreamExt;
    use std::sync::Arc;

    #[ignore]
    #[test]
    fn arbiter_world() {
        let environment = EnvironmentBuilder::new().build();
        let connection = Connection::from(&environment);
        let provider = Provider::new(connection);
        let mut world = World::new("test_world", provider);

        let client = RevmMiddleware::new(&environment, Some("testname")).unwrap();
        let mut agent = Agent::new("agent1");
        let messager = Messager::default();
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
