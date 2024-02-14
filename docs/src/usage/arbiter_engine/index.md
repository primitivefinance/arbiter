# Arbiter Engine
`arbiter-engine` provides the machinery to build agent based / event driven simulations and should be the primary entrypoint for using Arbiter.
The goal of this crate is to abstract away the work required to set up agents, their behaviors, and the worlds they live in.
At the moment, all interaction of agents is done through the `arbiter-core` crate and is meant to be for local simulations and it is not yet generalized for the case of live network automation.

## Hierarchy
The primary components of `arbiter-engine` are, from the bottom up:
- `Behavior<E>`: This is an event-driven behavior that takes in some item of type `E` and can act on that. 
The `Behavior<E>` has two methods: `startup` and `process`. 
    - `startup` is meant to initialize the `Behavior<E>` and any context around it.
    An example could be an agent that deploys token contracts on startup.
    - `process` is meant to be a stage that runs on every event that comes in.
    An example could be an agent that deployed token contracts on startup, and now wants to process queries about the tokens deployed in the simulation (e.g., what their addresses are).
- `Engine<B,E>` and `StateMachine`: The `Engine` is a struct that implements the `StateMachine` trait as an entrypoint to run `Behavior`s.
    - `Engine<B,E>` is a struct owns a `B: Behavior<E>` and the event stream `Stream<Item = E>` that the `Behavior<E>` will use for processing.
    - `StateMachine` is a trait that reduces the interface to `Engine<B,E>` to a single method: `execute`.
    This trait allows `Agent`s to have multiple behaviors that may not use the same event type.
- `Agent` a struct that contains an ID, a client (`Arc<RevmMiddleware>`) that provides means to send calls and transactions to an Arbiter `Environment`, and a `Messager`.
    - `Messager` is a struct that owns a `Sender` and `Receiver` for sending and receiving messages.
    This is a way for `Agent`s to communicate with each other.
    It can also be streamed and used for processing messages in a `Behavior<Message>`.
    - `Agent` also owns a `Vec<Box<dyn StateMachine>>` which is a list of `StateMachine`s that the `Agent` will run.
    This is a way for `Agent`s to have multiple `Behavior`s that may not use the same event type.
- `World` is a struct that has an ID, an Arbiter `Environment`, a mapping of `Agent`s, and a `Messager`.
    - The `World` is tasked with letting `Agent`s join in, and when they do so, to connect them to the `Environment` with a client and `Messager` with the `Agent`'s ID.
- `Universe` is a struct that wraps a mapping of `World`s.
    - The `Universe` is tasked with letting `World`s join in and running those `World`s in parallel.
