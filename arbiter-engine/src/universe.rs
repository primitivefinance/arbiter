//! The [`universe`] module contains the [`Universe`] struct which is the
//! primary interface for creating and running many `World`s in parallel.

use std::collections::HashMap;

use anyhow::anyhow;

use crate::world::World;

/// The [`Universe`] struct is the primary interface for creating and running
/// many `World`s in parallel. At the moment, is a wrapper around a
/// [`HashMap`] of [`World`]s, but can be extended to handle generics inside of
/// [`World`]s and crosstalk between [`World`]s.
#[derive(Debug, Default)]
pub struct Universe {
    worlds: Option<HashMap<String, World>>,
    world_tasks: Option<HashMap<String, tokio::task::JoinHandle<()>>>,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            worlds: Some(HashMap::new()),
            world_tasks: None,
        }
    }

    pub fn add_world(&mut self, world: World) {
        if let Some(worlds) = self.worlds.as_mut() {
            worlds.insert(world.id.clone(), world);
        }
    }

    pub fn run_worlds(&mut self) {
        let mut tasks = HashMap::new();
        for (id, mut world) in self.worlds.take().unwrap().drain() {
            tasks.insert(id, tokio::spawn(async move { world.run().await }));
        }
    }

    pub fn is_online(&self) -> bool {
        self.worlds.is_none() && self.world_tasks.is_some()
    }

    pub fn is_complete(&self, id: &str) -> anyhow::Result<bool> {
        if self.is_online() {
            let world_task = self
                .world_tasks
                .as_ref()
                .unwrap()
                .get(id)
                .ok_or(anyhow!("World not found."))?;
            Ok(world_task.is_finished())
        } else {
            Err(anyhow!("Universe is not online."))
        }
    }
}
