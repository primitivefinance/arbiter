const AGENT_ID: &str = "agent";

use super::*;
use crate::{agent::Behavior, messager::To};

struct TimedMessage {
    delay: u64,
    message: Message,
}

#[async_trait::async_trait]
impl Behavior for TimedMessage {
    #[tracing::instrument(skip(self), level = "trace")]
    async fn sync_state(&mut self) -> Result<()> {
        trace!("Syncing state for `TimedMessage`.");
        Ok(())
    }

    // TODO: Okay here is where the agent should NEVER have to know about the
    // `event.to` or anything even if the message is broadcast or direct.
    #[tracing::instrument(skip(self, event), level = "trace")]
    async fn process(&mut self) -> Vec<Message> {
        trace!("Processing event.");
        let message = Message {
            from: "agent".to_owned(),
            to: To::Agent("agent".to_owned()),
            data: "Hello, world!".to_owned(),
        };
        if event.data == "Start" {
            vec![message]
        } else {
            tokio::time::sleep(std::time::Duration::from_secs(self.delay)).await;
            vec![message]
        }
    }
}

// TODO: Can we combine the `world.run().await` through the `for task in tasks
// {task.await}` step to make this DEVX super easy TODO: Having something like
// an automatic impl of Start and Stop for all behaviors would be nice or load
// that in as a default behavior of agents or something.
#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn echoer() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();

    let environment = EnvironmentBuilder::new().build();
    let connection = Connection::from(&environment);
    let provider = Provider::new(connection);
    let mut world = World::new("test_world", provider);

    let agent = world.create_agent(AGENT_ID);

    let strategy = TimedMessage {
        delay: 1,
        message: Message {
            from: "agent".to_owned(),
            to: To::Agent("agent".to_owned()),
            data: "Hello, world!".to_owned(),
        },
    };
    let behavior = BehaviorBuilder::new()
        .add_collector(agent.messager.clone())
        .add_executor(agent.messager.clone())
        .add_strategy(strategy)
        .build();
    agent.add_behavior(behavior);

    debug!("Starting world.");
    let tasks = world.run().await;
    let message = Message {
        from: "agent".to_owned(),
        to: To::Agent("agent".to_owned()),
        data: "Start".to_owned(),
    };
    let send_result = world.messager.execute(message).await;
    debug!("Start message sent {:?}", send_result);

    world.join().await;
}
