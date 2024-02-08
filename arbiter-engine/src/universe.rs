//! The [`universe`] module contains the [`Universe`] struct which is the
//! primary interface for creating and running many `World`s in parallel.

use super::*;
use crate::world::World;

/// The [`Universe`] struct is the primary interface for creating and running
/// many `World`s in parallel. At the moment, is a wrapper around a
/// [`HashMap`] of [`World`]s, but can be extended to handle generics inside of
/// [`World`]s and crosstalk between [`World`]s.
#[derive(Debug, Default)]
pub struct Universe {
    worlds: Option<HashMap<String, World>>,
    world_tasks: Option<Vec<Result<World, JoinError>>>,
}

impl Universe {
    /// Creates a new [`Universe`].
    pub fn new() -> Self {
        Self {
            worlds: Some(HashMap::new()),
            world_tasks: None,
        }
    }

    /// Adds a [`World`] to the [`Universe`].
    pub fn add_world(&mut self, world: World) {
        if let Some(worlds) = self.worlds.as_mut() {
            worlds.insert(world.id.clone(), world);
        }
    }

    /// Runs all of the [`World`]s in the [`Universe`] in parallel.
    pub async fn run_worlds(&mut self) -> Result<(), ArbiterEngineError> {
        if self.is_online() {
            return Err(ArbiterEngineError::UniverseError(
                "Universe is already running.".to_owned(),
            ));
        }
        let mut tasks = Vec::new();
        // NOTE: Unwrap is safe because we checked if the universe is online.
        for (_, mut world) in self.worlds.take().unwrap().drain() {
            tasks.push(spawn(async move {
                world.run().await.unwrap();
                world
            }));
        }
        self.world_tasks = Some(join_all(tasks.into_iter()).await);
        Ok(())
    }

    /// Returns `true` if the [`Universe`] is running.
    pub fn is_online(&self) -> bool {
        self.world_tasks.is_some()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{read_to_string, remove_file, File};

    use tracing_subscriber::{fmt, EnvFilter};

    use super::*;
    use crate::{agent::Agent, examples::timed_message::*};

    #[tokio::test]
    async fn run_universe() {
        let mut universe = Universe::new();
        let world = World::new("test");
        universe.add_world(world);
        universe.run_worlds().await.unwrap();
        universe.world_tasks.unwrap().remove(0).unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Universe is already running.")]
    async fn cant_run_twice() {
        let mut universe = Universe::new();
        let world1 = World::new("test");
        universe.add_world(world1);
        universe.run_worlds().await.unwrap();
        universe.run_worlds().await.unwrap();
    }

    #[tokio::test]
    async fn run_parallel() {
        std::env::set_var("RUST_LOG", "trace");
        let file = File::create("test_logs_engine.log").expect("Unable to create log file");

        let subscriber = fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_writer(file)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let mut world1 = World::new("test1");
        let agent1 = Agent::builder("agent1");
        let behavior1 = TimedMessage::new(
            1,
            "echo".to_owned(),
            "echo".to_owned(),
            Some(5),
            Some("echo".to_owned()),
        );
        world1.add_agent(agent1.with_behavior(behavior1));

        let mut world2 = World::new("test2");
        let agent2 = Agent::builder("agent2");
        let behavior2 = TimedMessage::new(
            1,
            "echo".to_owned(),
            "echo".to_owned(),
            Some(5),
            Some("echo".to_owned()),
        );
        world2.add_agent(agent2.with_behavior(behavior2));

        let mut universe = Universe::new();
        universe.add_world(world1);
        universe.add_world(world2);

        universe.run_worlds().await.unwrap();

        let parsed_file = read_to_string("test_logs_engine.log").expect("Unable to read log file");

        // Define the line to check (excluding the timestamp)
        let line_to_check = "Behavior is starting up.";

        // Assert that the lines appear consecutively
        assert!(
            lines_appear_consecutively(&parsed_file, line_to_check),
            "The lines do not appear consecutively"
        );
        remove_file("test_logs_engine.log").expect(
            "Unable to remove log
        file",
        );
    }

    fn lines_appear_consecutively(file_contents: &str, line_to_check: &str) -> bool {
        let mut lines = file_contents.lines();

        while let Some(line) = lines.next() {
            if line.contains(line_to_check) {
                println!("Found line: {}", line);
                // Check if the next line also contains the line_to_check
                if let Some(next_line) = lines.next() {
                    if next_line.contains(line_to_check) {
                        println!("Found next line: {}", next_line);
                        return true;
                    }
                }
            }
        }

        false
    }
}
