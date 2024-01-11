#[cfg(test)]

const AGENT_ID: &str = "agent";

use super::*;
use crate::{
    machine::{Behavior, Engine, State, StateMachine},
    messager::To,
    world::World,
};

struct TimedMessage {
    delay: u64,
    message: Message,
    messager: Messager,
}

#[async_trait::async_trait]
impl Behavior<Message> for TimedMessage {
    async fn process(&mut self, event: Message) {
        trace!("Processing event.");
        if event.data != self.message.data {
            return;
        } else {
            trace!("Event matches message. Sending a new message.");
            self.messager.send(self.message.clone()).await;
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

    let messager = world.messager.clone();

    let agent = world.create_agent(AGENT_ID);

    let message = Message {
        from: "agent".to_owned(),
        to: To::Agent("agent".to_owned()),
        data: "Hello, world!".to_owned(),
    };

    let behavior = TimedMessage {
        delay: 1,
        message: message.clone(),
        messager,
    };
    agent.add_behavior(behavior);

    tracing::debug!("Starting world.");
    let messager = world.messager.clone();
    world.run_state(State::Syncing);
    world.transition().await;

    world.run_state(State::Startup);
    world.transition().await;

    world.run_state(State::Processing);

    messager.send(message).await;
    tracing::debug!("Start message sent from main thread.");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    world.transition().await;
}
