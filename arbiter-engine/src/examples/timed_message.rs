use crate::agent::BehaviorBuilder;

use super::*;

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

#[ignore]
#[tokio::test]
async fn echoer() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE) // Set the maximum level to TRACE
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

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
    let world_task = tokio::spawn(async move { world.run().await });
    let message = Message {
        from: "agent".to_owned(),
        to: "agent".to_owned(),
        data: "Start".to_owned(),
    };
    let _send_result = messager.execute(message).await;

    world_task.await.unwrap();
}
