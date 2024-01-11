#[cfg(test)]

const AGENT_ID: &str = "agent";

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
}

#[async_trait::async_trait]
impl Behavior<Message> for TimedMessage {
    async fn process(&mut self, event: Message) {
        trace!("Processing event.");
        if event.data != self.receive_data {
            return;
        } else {
            trace!("Event matches message. Sending a new message.");
            let message = Message {
                from: self.messager.id.clone().unwrap(),
                to: To::All,
                data: self.send_data.clone(),
            };
            self.messager.send(message).await;
        }

        tokio::time::sleep(std::time::Duration::from_secs(self.delay)).await;
        // TODO: send a message
        trace!("Processed event.");
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

// // TODO: Can we combine the `world.run().await` through the `for task in
// tasks // {task.await}` step to make this DEVX super easy TODO: Having
// something like // an automatic impl of Start and Stop for all behaviors
// would  be nice or load // that in as a default behavior of agents or
// something.
#[ignore = "This is a work in progress and does not work and does not ever terminate."]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn echoer() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();

    let mut world = World::new("world");

    let agent = Agent::new(AGENT_ID, &world);
    let behavior = TimedMessage {
        delay: 1,
        receive_data: "Hello, world!".to_owned(),
        send_data: "Hello, world!".to_owned(),
        messager: agent.messager.clone(),
    };
    world.add_agent(agent.with_behavior(behavior));

    tracing::debug!("Starting world.");
    let messager = world.messager.clone();
    world.run_state(State::Syncing);
    world.transition().await;

    world.run_state(State::Startup);
    world.transition().await;

    world.run_state(State::Processing);

    let message = Message {
        from: "god".to_owned(),
        to: To::Agent("agent".to_owned()),
        data: "Hello, world!".to_owned(),
    };
    world.messager.send(message).await;
    tracing::debug!("Start message sent from main thread.");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    world.transition().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ping_pong() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();

    let mut world = World::new("world");

    let agent = Agent::new(AGENT_ID, &world);
    let behavior_ping = TimedMessage {
        delay: 1,
        receive_data: "pong".to_owned(),
        send_data: "ping".to_owned(),
        messager: agent.messager.clone(),
    };
    let behavior_pong = TimedMessage {
        delay: 1,
        receive_data: "ping".to_owned(),
        send_data: "pong".to_owned(),
        messager: agent.messager.clone(),
    };

    world.add_agent(
        agent
            .with_behavior(behavior_ping)
            .with_behavior(behavior_pong),
    );

    tracing::debug!("Starting world.");
    world.run_state(State::Syncing);
    world.transition().await;

    world.run_state(State::Startup);
    world.transition().await;

    world.run_state(State::Processing);

    let init_message = Message {
        from: "god".to_owned(),
        to: To::Agent("agent".to_owned()),
        data: "ping".to_owned(),
    };
    world.messager.send(init_message).await;
    tracing::debug!("Start message sent from main thread.");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    world.transition().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ping_pong_two_agent() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();

    let mut world = World::new("world");

    let agent_ping = Agent::new("agent_ping", &world);
    let behavior_ping = TimedMessage {
        delay: 1,
        receive_data: "pong".to_owned(),
        send_data: "ping".to_owned(),
        messager: agent_ping.messager.clone(),
    };

    let agent_pong = Agent::new("agent_pong", &world);
    let behavior_pong = TimedMessage {
        delay: 1,
        receive_data: "ping".to_owned(),
        send_data: "pong".to_owned(),
        messager: agent_pong.messager.clone(),
    };

    world.add_agent(agent_ping.with_behavior(behavior_ping));
    world.add_agent(agent_pong.with_behavior(behavior_pong));

    tracing::debug!("Starting world.");
    let messager = world.messager.clone();
    world.run_state(State::Syncing);
    world.transition().await;

    world.run_state(State::Startup);
    world.transition().await;

    world.run_state(State::Processing);

    let init_message = Message {
        from: "god".to_owned(),
        to: To::All,
        data: "ping".to_owned(),
    };

    world.messager.send(init_message).await;
    tracing::debug!("Start message sent from main thread.");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    world.transition().await;
}
