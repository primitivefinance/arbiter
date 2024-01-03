use super::*;
use crate::agent::BehaviorBuilder;

struct TimedMessage {
    delay: u64,
    message: Message,
}

#[async_trait::async_trait]
impl Strategy<Message, Message> for TimedMessage {
    #[tracing::instrument(skip(self), level = "trace")]
    async fn sync_state(&mut self) -> Result<()> {
        trace!("Syncing state for `TimedMessage`.");
        Ok(())
    }

    #[tracing::instrument(skip(self, event), level = "trace")]
    async fn process_event(&mut self, event: Message) -> Vec<Message> {
        trace!("Processing event.");
        if event.to == self.message.to {
            let message = Message {
                from: "agent".to_owned(),
                to: "agent".to_owned(),
                data: "Hello, world!".to_owned(),
            };
            if event.data == "Start" {
                vec![message]
            } else {
                tokio::time::sleep(std::time::Duration::from_secs(self.delay)).await;
                vec![message]
            }
        } else {
            vec![]
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
    let messager = world.messager.clone();

    let mut agent = Agent::new("agent");
    let strategy = TimedMessage {
        delay: 1,
        message: Message {
            from: "agent".to_owned(),
            to: "agent".to_owned(),
            data: "Hello, world!".to_owned(),
        },
    };
    let behavior = BehaviorBuilder::new()
        .add_collector(messager.clone())
        .add_executor(messager.clone())
        .add_strategy(strategy)
        .build();
    agent.add_behavior(behavior);
    world.add_agent(agent);

    debug!("Starting world.");
    let tasks = world.run().await;
    let message = Message {
        from: "agent".to_owned(),
        to: "agent".to_owned(),
        data: "Start".to_owned(),
    };
    let send_result = messager.execute(message).await;
    debug!("Start message sent {:?}", send_result);

    for task in tasks {
        task.await.unwrap();
    }
}
