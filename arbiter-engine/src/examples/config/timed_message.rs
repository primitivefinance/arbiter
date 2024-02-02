use super::*;
use crate::{examples::timed_message::TimedMessage, world::World};

enum Behaviors {
    TimedMessage(TimedMessage),
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn config_test() {
    let mut world = World::new("world");
    world.build_with_config("./test_config.toml").run().await;
}
