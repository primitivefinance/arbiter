# Agents and Engines
`Behavior`s are the heartbeat of your `Agent`s and they are wrapped by `Engine`s. 
The main idea here is that you can have an `Agent` that has as many `Behavior`s as you like, and each of those behaviors may process different types of events.
This gives flexibility in how you want to design your `Agent`s and what emergent properties you want to observe.

## Design Principles
We designed the behaviors to be flexible. It is up to you whether or not you prefer to have `Agent`s have multiple `Behavior`s or not or if you want them to have a single `Behavior` that processes all events.
For the former case, you will build `Behavior<E>` for different types `E` and place these inside of an `Agent`.
For the latter, you will create an `enum` that wraps all the different types of events that you want to process and then implement `Behavior` on that `enum`.
The latter will also require a `stream::select` type of operation to merge all the different event streams into one, though this is not difficult to do.

## `struct Agent`
The `Agent` struct is the primary struct that you will be working with.
It contains an ID, a client (`Arc<RevmMiddleware>`) that provides means to send calls and transactions to an Arbiter `Environment`, and a `Messager`.
It looks like this:
```rust, ignore
pub struct Agent {
    pub id: String,
    pub messager: Messager,
    pub client: Arc<RevmMiddleware>,
    pub(crate) behavior_engines: Vec<Box<dyn StateMachine>>,
}
```

Your work will only be to define `Behavior`s and then add them to an `Agent` with the `Agent::with_behavior` method.

The `Agent` is inactive until it is paired with a `World` and then it is ready to be run.
This is handled by creating a world (see: [Worlds and Universes](./worlds_and_universes.md)) and then adding the `Agent` to the `World` with the `World::add_agent` method.
Some of the intermediary representations are below:

#### `struct AgentBuilder`
The `AgentBuilder` struct is a builder pattern for creating `Agent`s.
This is essentially invisible for the end-user, but it is used internally so that `Agent`s can be built in a more ergonomic way.

#### `struct Engine<B,E>`
Briefly, the `Engine<B,E>` struct provides the machinery to run a `Behavior<E>` and it is not necessary for you to handle this directly. 
The purpose of this design is to encapsulate the `Behavior<E>` and the event stream `Stream<Item = E>` that the `Behavior<E>` will use for processing.
This encapsulation also allows the `Agent` to hold onto `Behavior<E>` for various different types of `E` all at once.

## Example
Let's create an `Agent` that has two `Behavior`s using the `Replier` behavior from before.
```rust, ignore
use arbiter_engine::agent::Agent;
use crate::Replier;

fn setup() {
    let ping_replier = Replier::new("ping", "pong", 5, None);
    let pong_replier = Replier::new("pong", "ping", 5, Some("ping"));
    let agent = Agent::builder("my_agent")
                    .with_behavior(ping_replier)
                    .with_behavior(pong_replier);
}
```
In this example, we have created an `Agent` with two `Replier` behaviors.
The `ping_replier` will reply to a message with "pong" and the `pong_replier` will reply to a message with "ping".
Given that the `pong_replier` has a `startup_message` of "ping", it will send a message to everyone (including the "my_agent" itself who holds the `ping_replier` behavior) when it starts up.
This will start a chain of messages that will continue in a "ping" "pong" fashion until the `max_count` is reached.
```
