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
}

#[async_trait::async_trait]
impl Behavior<Message> for TimedMessage {
    async fn process(&mut self, event: Message) {
        trace!("Processing event.");
        let message = Message {
            from: "agent".to_owned(),
            to: To::Agent("agent".to_owned()),
            data: "Hello, world!".to_owned(),
        };

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
#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn echoer() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();

    let mut world = World::new("world");

    let agent = world.create_agent(AGENT_ID);

    let behavior = TimedMessage {
        delay: 2,
        message: Message {
            from: "agent".to_owned(),
            to: To::Agent("agent".to_owned()),
            data: "Hello, world!".to_owned(),
        },
    };
    agent.add_behavior(behavior);

    debug!("Starting world.");
    let messager = world.messager.clone();
    world.run_state(State::Syncing);
    world.transition().await;

    world.run_state(State::Startup);
    world.transition().await;

    world.run_state(State::Processing);

    let message = Message {
        from: "agent".to_owned(),
        to: To::Agent("agent".to_owned()),
        data: "Start".to_owned(),
    };
    let send_result = messager.send(message).await;
    debug!("Start message sent {:?}", send_result);

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    world.transition().await;
}
