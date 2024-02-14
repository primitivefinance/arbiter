# Worlds and Universes
`Universes` are the top-level struct that you will be working with in the Arbiter Engine.
They are tasked with letting `World`s join in and running those `World`s in parallel.
By no means are you required to use `Universe`s, but they will be useful for running multiple simulations at once or, in the future, they will allow for running `World`s that have different internal environments.
For instance, one could have a `World` that consists of `Agent`s acting on the Ethereum mainnet, another `World` that consists of `Agent`s acting on Optimism, and finally a `World` that has an Arbiter `Environment` as the network analogue.
Using these in tandem is a long-term goal of the Arbiter project.

Depending on your needs, you will either use the `Universe` if you want to run multiple `World`s in parallel or you will use the `World` if you only want to run a single simulation.
The choice is yours.

## `struct Universe`
The `Universe` struct looks like this:
```rust, ignore
pub struct Universe {
    worlds: Option<HashMap<String, World>>,
    world_tasks: Option<Vec<Result<World, JoinError>>>,
}
```
The `Universe` is a struct that wraps a mapping of `World`s where the key of the map is the `World`'s ID.
Also, the `Universe` manages the running of those `World`s in parallel by storing the running `World`s as tasks.
In the future, more introspection and control will be added to the `Universe` to allow for debugging and managing the running `World`s.

The `Universe::run_worlds` currently iterates through the `World`s and starts them in concurrent tasks.

## `struct World`
The `World` struct looks like this:
```rust, ignore
pub struct World {
    pub id: String,
    pub agents: Option<HashMap<String, Agent>>,
    pub environment: Environment,
    pub messager: Messager,
}
```
The `World` is a struct that has an ID, an Arbiter `Environment`, a mapping of `Agent`s, and a `Messager`.
The `World` is tasked with letting `Agent`s join in, and when they do so, to connect them to the `Environment` with a client and `Messager` with the `Agent`'s ID.
Then the `World` stores the `Agent`s in a map where the key is the `Agent`'s ID.

The main methods to use with the world is `World::add_agent` which adds an agent to the `World` and `World::run` which will engage all of the `Agent` `Behavior`s.

In future development, the `World` will be generic over your choice of `Provider` that encapsulates the Ethereum-like execution environment you want to use (e.g., Ethereum mainnet, Optimism, or an Arbiter `Environment`).

## Example
Let's first do a quick example where we take a `World` and add an `Agent` to it.
```rust, ignore
use arbiter_engine::{agent::Agent, world::World};
use crate::Replier;

fn setup_world(id: &str) -> World {
    let ping_replier = Replier::new("ping", "pong", 5, None);
    let pong_replier = Replier::new("pong", "ping", 5, Some("ping"));
    let agent = Agent::new("my_agent")
                    .with_behavior(ping_replier)
                    .with_behavior(pong_replier);
    let mut world = World::new(id);
    world.add_agent(agent);
}

async fn run() {
    let world = setup_world("my_world");
    world.run().await;
}
```
If you wanted to extend this to use a `Universe`, you would simply create a `Universe` and add the `World` to it.
```rust, ignore
use arbiter_engine::{agent::Agent, world::World};
use crate::Replier;

fn setup_world(id: &str) -> World {
    let ping_replier = Replier::new("ping", "pong", 5, None);
    let pong_replier = Replier::new("pong", "ping", 5, Some("ping"));
    let agent = Agent::new("my_agent")
                    .with_behavior(ping_replier)
                    .with_behavior(pong_replier);
    let mut world = World::new(id);
    world.add_agent(agent);
}

fn main() {
    let mut universe = Universe::new();
    universe.add_world(setup_world("my_world"));
    universe.add_world(setup_world("my_other_world"));
    universe.run_worlds().await;
}
```
