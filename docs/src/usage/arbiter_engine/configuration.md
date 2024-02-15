# Configuration
To make it so you rarely have to recompile your project, you can use a configuration file to set the parameters of your simulation once your `Behavior`s have been defined.
Let's take a look at how to do this.

## Behavior Enum
It is good practice to take your `Behavior`s and wrap them in an `enum` so that you can use them in a configuration file.
For instance, let's say you have two struct `Maker` and `Taker` that implement `Behavior<E>` for their own `E`.
Then you can make your `enum` like this:
```rust, ignore
use arbiter_macros::Behaviors;

#[derive(Behaviors)]
pub enum Behaviors {
    Maker(Maker),
    Taker(Taker),
}
```
Notice that we used the `Behaviors` derive macro from the `arbiter_macros` crate.
This macro will generate an implementation of a `CreateStateMachine` trait for the `Behaviors` enum and ultimately save you from having to write a lot of boilerplate code.
The macro solely requires that the `Behavior`s you have implement the `Behavior` trait and that the necessary imports are in scope.

## Configuration File
Now that you have your `enum` of `Behavior`s, you can configure your `World` and the `Agent`s inside of it from configuration file.
Since the `World` and your simulation is completely defined by the `Agent` `Behavior`s you make, all you need to do is specify your `Agent`s in the configuration file.
For example, let's say we have the `Replier` behavior from before, so we have:
```rust, ignore
#[derive(Behaviors)]
pub enum Behaviors {
    Replier(Replier),
}

pub struct Replier {
    receive_data: String,
    send_data: String,
    max_count: u64,
    startup_message: Option<String>,
    count: u64,
    messager: Option<Messager>,
}
```
Then, we can specify the "ping" and "pong" `Behavior`s like this:
```toml
[[my_agent]]
Replier = { send_data = "ping", receive_data = "pong", max_count = 5, startup_message = "ping" }

[[my_agent]]
Replier = { send_data = "pong", receive_data = "ping", max_count = 5 }
```
If you instead wanted to specify two `Agent`s "Alice" and "Bob" each with one of the `Replier` `Behavior`s, you could do it like this:
```toml
[[alice]]
Replier = { send_data = "ping", receive_data = "pong", max_count = 5, startup_message = "ping" }

[[bob]]
Replier = { send_data = "pong", receive_data = "ping", max_count = 5 }
```

## Loading the Configuration
Once you have your configuration file located at `./path/to/config.toml`, you can load it and run your simulation like this:
```rust, ignore
fn main()  {
    let world = World::from_config("./path/to/config.toml")?;
    world.run().await;
}
```
At the moment, we do not configure `Universe`s from a configuration file, but this is a feature that is planned for the future.