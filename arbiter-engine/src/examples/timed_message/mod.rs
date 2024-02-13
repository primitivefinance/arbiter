use super::*;

const AGENT_ID: &str = "agent";

use std::{pin::Pin, time::Duration};

use anyhow::Result;
use ethers::types::BigEndianHash;
use futures_util::Stream;
use serde::*;
use tokio::time::timeout;

use super::*;

fn default_max_count() -> Option<u64> {
    Some(3)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TimedMessage {
    delay: u64,
    receive_data: String,
    send_data: String,
    #[serde(skip)]
    messager: Option<Messager>,
    #[serde(default)]
    count: u64,
    #[serde(default = "default_max_count")]
    max_count: Option<u64>,
    startup_message: Option<String>,
}

impl TimedMessage {
    pub fn new(
        delay: u64,
        receive_data: String,
        send_data: String,
        max_count: Option<u64>,
        startup_message: Option<String>,
    ) -> Self {
        Self {
            delay,
            receive_data,
            send_data,
            messager: None,
            count: 0,
            max_count,
            startup_message,
        }
    }
}

#[async_trait::async_trait]
impl Behavior<Message> for TimedMessage {
    async fn startup(
        &mut self,
        _client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<EventStream<Message>> {
        if let Some(startup_message) = &self.startup_message {
            messager.send(To::All, startup_message).await;
        }
        self.messager = Some(messager.clone());
        Ok(messager.stream()?)
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        if event.data == serde_json::to_string(&self.receive_data).unwrap() {
            let messager = self.messager.clone().unwrap();
            messager.send(To::All, self.send_data.clone()).await;
            self.count += 1;
        }
        if self.count == self.max_count.unwrap_or(u64::MAX) {
            return Ok(ControlFlow::Halt);
        }
        Ok(ControlFlow::Continue)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn echoer() {
    let mut world = World::new("world");

    let agent = Agent::builder(AGENT_ID);
    let behavior = TimedMessage::new(
        1,
        "Hello, world!".to_owned(),
        "Hello, world!".to_owned(),
        Some(2),
        Some("Hello, world!".to_owned()),
    );
    world.add_agent(agent.with_behavior(behavior));
    let messager = world.messager.for_agent("outside_world");

    world.run().await;

    let mut stream = messager.stream().unwrap();
    let mut idx = 0;

    loop {
        match timeout(Duration::from_secs(1), stream.next()).await {
            Ok(Some(event)) => {
                println!("Event received in outside world: {:?}", event);
                idx += 1;
                if idx == 2 {
                    break;
                }
            }
            _ => {
                panic!("Timeout reached. Test failed.");
            }
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ping_pong() {
    let mut world = World::new("world");

    let agent = Agent::builder(AGENT_ID);
    let behavior_ping = TimedMessage::new(
        1,
        "pong".to_owned(),
        "ping".to_owned(),
        Some(2),
        Some("ping".to_owned()),
    );
    let behavior_pong = TimedMessage::new(1, "ping".to_owned(), "pong".to_owned(), Some(2), None);

    world.add_agent(
        agent
            .with_behavior(behavior_ping)
            .with_behavior(behavior_pong),
    );

    let messager = world.messager.for_agent("outside_world");
    world.run().await;

    let mut stream = messager.stream().unwrap();
    let mut idx = 0;

    loop {
        match timeout(Duration::from_secs(1), stream.next()).await {
            Ok(Some(event)) => {
                println!("Event received in outside world: {:?}", event);
                idx += 1;
                if idx == 4 {
                    break;
                }
            }
            _ => {
                panic!("Timeout reached. Test failed.");
            }
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ping_pong_two_agent() {
    let mut world = World::new("world");

    let agent_ping = Agent::builder("agent_ping");
    let agent_pong = Agent::builder("agent_pong");

    let behavior_ping = TimedMessage::new(
        1,
        "pong".to_owned(),
        "ping".to_owned(),
        Some(2),
        Some("ping".to_owned()),
    );
    let behavior_pong = TimedMessage::new(1, "ping".to_owned(), "pong".to_owned(), Some(2), None);

    world.add_agent(agent_ping.with_behavior(behavior_ping));
    world.add_agent(agent_pong.with_behavior(behavior_pong));

    let messager = world.messager.for_agent("outside_world");
    world.run().await;

    let mut stream = messager.stream().unwrap();
    let mut idx = 0;

    loop {
        match timeout(Duration::from_secs(1), stream.next()).await {
            Ok(Some(event)) => {
                println!("Event received in outside world: {:?}", event);
                idx += 1;
                if idx == 5 {
                    break;
                }
            }
            _ => {
                panic!("Timeout reached. Test failed.");
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Behaviors)]
enum Behaviors {
    TimedMessage(TimedMessage),
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn config_test() {
    let mut world = World::new("world");
    world.from_config::<Behaviors>("src/examples/timed_message/config.toml");

    world.run().await;
}
