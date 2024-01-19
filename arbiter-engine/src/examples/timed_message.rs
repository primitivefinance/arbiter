#[cfg(test)]

const AGENT_ID: &str = "agent";

use self::machine::MachineHalt;
use super::*;
use crate::{
    agent::Agent,
    machine::{Behavior, Engine, State, StateMachine},
    messager::To,
    world::World,
};

struct TimedMessage {
    delay: u64,
    receive_data: String,
    send_data: String,
    messager: Messager,
    count: u64,
    max_count: Option<u64>,
}

#[async_trait::async_trait]
impl Behavior<Message> for TimedMessage {
    async fn process(&mut self, event: Message) -> Option<MachineHalt> {
        trace!("Processing event.");
        if event.data == self.receive_data {
            trace!("Event matches message. Sending a new message.");
            let message = Message {
                from: self.messager.id.clone().unwrap(),
                to: To::All,
                data: self.send_data.clone(),
            };
            self.messager.send(message).await;
            self.count += 1;
        }
        if self.count == self.max_count.unwrap_or(u64::MAX) {
            return Some(MachineHalt);
        }

        tokio::time::sleep(std::time::Duration::from_secs(self.delay)).await;
        trace!("Processed event.");
        None
    }

    async fn sync(&mut self) {
        trace!("Syncing state for `TimedMessage`.");
        tokio::time::sleep(std::time::Duration::from_secs(self.delay)).await;
        trace!("Synced state for `TimedMessage`.");
    }

    async fn startup(&mut self) {
        trace!("Starting up `TimedMessage`.");
        tokio::time::sleep(std::time::Duration::from_secs(self.delay)).await;
        trace!("Started up `TimedMessage`.");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn echoer() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();

    let mut world = World::new("world");

    let agent = Agent::new(AGENT_ID, &world);
    let behavior = TimedMessage {
        delay: 1,
        receive_data: "Hello, world!".to_owned(),
        send_data: "Hello, world!".to_owned(),
        messager: agent
            .messager
            .as_ref()
            .unwrap()
            .join_with_id(Some(AGENT_ID.to_owned())),
        count: 0,
        max_count: Some(5),
    };
    world.add_agent(agent.with_behavior(behavior));

    let messager = world.messager.join_with_id(Some("god".to_owned()));
    let task = world.run();

    let message = Message {
        from: "god".to_owned(),
        to: To::Agent("agent".to_owned()),
        data: "Hello, world!".to_owned(),
    };
    messager.send(message).await;
    task.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ping_pong() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();

    let mut world = World::new("world");

    let agent = Agent::new(AGENT_ID, &world);
    let behavior_ping = TimedMessage {
        delay: 1,
        receive_data: "pong".to_owned(),
        send_data: "ping".to_owned(),
        messager: agent
            .messager
            .as_ref()
            .unwrap()
            .join_with_id(Some(AGENT_ID.to_owned())),
        count: 0,
        max_count: Some(5),
    };
    let behavior_pong = TimedMessage {
        delay: 1,
        receive_data: "ping".to_owned(),
        send_data: "pong".to_owned(),
        messager: agent
            .messager
            .as_ref()
            .unwrap()
            .join_with_id(Some(AGENT_ID.to_owned())),
        count: 0,
        max_count: Some(5),
    };

    world.add_agent(
        agent
            .with_behavior(behavior_ping)
            .with_behavior(behavior_pong),
    );

    let messager = world.messager.join_with_id(Some("god".to_owned()));
    let task = world.run();

    let init_message = Message {
        from: "god".to_owned(),
        to: To::Agent("agent".to_owned()),
        data: "ping".to_owned(),
    };
    messager.send(init_message).await;

    task.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ping_pong_two_agent() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();

    let mut world = World::new("world");

    let agent_ping = Agent::new("agent_ping", &world);
    let behavior_ping = TimedMessage {
        delay: 1,
        receive_data: "pong".to_owned(),
        send_data: "ping".to_owned(),
        messager: agent_ping
            .messager
            .as_ref()
            .unwrap()
            .join_with_id(Some("agent_ping".to_owned())),
        count: 0,
        max_count: Some(5),
    };

    let agent_pong = Agent::new("agent_pong", &world);
    let behavior_pong = TimedMessage {
        delay: 1,
        receive_data: "ping".to_owned(),
        send_data: "pong".to_owned(),
        messager: agent_pong
            .messager
            .as_ref()
            .unwrap()
            .join_with_id(Some("agent_pong".to_owned())),
        count: 0,
        max_count: Some(5),
    };

    world.add_agent(agent_ping.with_behavior(behavior_ping));
    world.add_agent(agent_pong.with_behavior(behavior_pong));

    let messager = world.messager.join_with_id(Some("god".to_owned()));
    let task = world.run();

    let init_message = Message {
        from: "god".to_owned(),
        to: To::All,
        data: "ping".to_owned(),
    };

    messager.send(init_message).await;

    task.await;
}
