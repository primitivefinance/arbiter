# Behaviors
The design of `arbiter-engine` is centered around the concept of `Agent`s and `Behavior`s.
At the core, we place `Behavior`s as the event-driven machinery that defines the entire simulation.
What we want is that your simulation is defined completely with how your `Agent`s behaviors are defined.
All you should be looking for is how to define your `Agent`s behaviors and what emergent properties you want to observe.

## `trait Behavior<E>`
To define a `Behavior`, you need to implement the `Behavior` trait on a struct of your own design.
The `Behavior` trait is defined as follows:
```rust, ignore
pub trait Behavior<E> {
    fn startup(&mut self, client: Arc<RevmMiddleware>, messager: Messager) -> Result<EventStream<E>, ArbiterEngineError>;
    fn process(&mut self, event: E) -> Result<ControlFlow, ArbiterEngineError>;
}
```
To outline the design principles here:
- `startup` is a method that initializes the `Behavior` and returns an `EventStream` that the `Behavior` will use for processing.
    - This method yields a client and messager from the `Agent` that owns the `Behavior`.
    In this method you should take the client and messager and store them in your struct if you will need them in the processing of events.
    Note, you may not need them!
- `process` is a method that processes an event of type `E` and returns an `Option<MachineHalt>`. 
    - If `process` returns `Some(MachineHalt)`, then the `Behavior` will stop processing events completely.

**Summary:** A `Behavior<E>` is tantamount to engage the processing some events of type `E`.

**Advice:** `Behavior`s should be limited in scope and should be a simplistic action driven from a single event.
Otherwise you risk having a `Behavior` that is too complex and difficult to understand and maintain.

### Example
To see this in use, let's take a look at an example of a `Behavior` called `Replier` that replies to a message with a message of its own, and stops once it has replied a certain number of times.
```rust, ignore
use std::sync::Arc;
use arbiter_core::middleware::RevmMiddleware;
use arbiter_engine::{
    machine::{Behavior, ControlFlow},
    messager::{Messager, To}, 
    EventStream};

pub struct Replier {
    receive_data: String,
    send_data: String,
    max_count: u64,
    startup_message: Option<String>,
    count: u64,
    messager: Option<Messager>,
}

impl Replier {
    pub fn new(
        receive_data: String,
        send_data: String,
        max_count: u64,
        startup_message: Option<String>,
    ) -> Self {
        Self {
            receive_data,
            send_data,
            startup_message,
            max_count,
            count: 0,
            messager: None,
        }
    }
}

impl Behavior<Message> for Replier {
    async fn startup(
        &mut self,
        client: Arc<RevmMiddleware>,
        messager: Messager,
    ) -> Result<EventStream<Message>, ArbiterEngineError> {
        if let Some(startup_message) = &self.startup_message {
            messager.send(To::All, startup_message).await;
        }
        self.messager = Some(messager.clone());
        messager.stream()
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow, ArbiterEngineError> {
        if event.data == self.receive_data {
            self.messager.unwrap().messager.send(To::All, send_data).await;
            self.count += 1;
        }
        if self.count == self.max_count {
            return Ok(ControlFlow::Halt);
        }
        Ok(ControlFlow::Continue)
    }
}
```
In this example, we have a `Behavior` that upon `startup` will see if there is a `startup_message` assigned and if so, send it to all `Agent`s that are listening to their `Messager`.
Then, it will store the `Messager` for sending messages later on and start a stream of incoming messages so that we have `E = Message` in this case.
Once these are completed, the `Behavior` automatically transitions into the `process`ing stage where events are popped from the `EventStream<E>` and fed to the `process` method.

As messages come in, if the `receive_data` matches the incoming message, then the `Behavior` will send the `send_data` to all `Agent`s listening to their `Messager` a message with data `send_data`.