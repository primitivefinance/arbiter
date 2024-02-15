# Arbiter macros
`arbiter_macros` provides a set of macros to help with the use of `arbiter-engine` and `arbiter-core`.
Macros allow for code generation which enables developers to write code that writes code. 
We use them here to reduce boilerplate by abstracting repetitive patterns. 
Macros can be used for tasks like deriving traits automatically or for generating code based on custom attributes.

## Procedural Macros

> **`#[derive(Behaviors)]`**
This Rust procedural macro automatically implements the [CreateStateMachine](https://github.com/primitivefinance/arbiter/blob/ffbbd146dc05f3e1088a9df5cf78452a1bef2212/macros/src/lib.rs#L68) trait for an enum, generating a [create_state_machine](https://github.com/primitivefinance/arbiter/blob/ffbbd146dc05f3e1088a9df5cf78452a1bef2212/macros/src/lib.rs#L26) method that matches each enum variant to a new state machine instance. 
It's designed for enums where each variant contains a single unnamed field representing state data. 
This macro simplifies the creation of state machines from enums, eliminating repetitive boilerplate code and enhancing code maintainability. 

### Example
You can use this macro like so:
```rust, ignore
use arbiter_macros::Behaviors;
use arbiter_engine::machine::Behavior;

struct MyBehavior1 {}
impl Behavior for MyBehavior1 {
    // ...
}
struct MyBehavior2 {}

}
impl Behavior for MyBehavior2 {
    // ...
}

#[derive(Behaviors)]
enum Behaviors {
    MyBehavior1(MyBehavior1),
    MyBehavior2(MyBehavior2),
}
```

> **`#[main]`**.
The [`#[arbiter_macros::main]`](https://github.com/primitivefinance/arbiter/blob/ffbbd146dc05f3e1088a9df5cf78452a1bef2212/macros/src/lib.rs#L161) macro in `arbiter-macros/src/lib.rs` is designed to simplify the creation of a CLI that will let you run your simulations by automatically generating a `main` function that sets up command-line parsing, logging, async execution, and world creation.
It takes custom attributes to configure the application's metadata such as the project's name, description, and the set of behaviors you want to use.
Under the hood, it uses the [clap](https://crates.io/crates/clap) crate for parsing CLI arguments and [tracing](https://crates.io/crates/tracing) for logging based on verbosity level. 
The macro needs to have have an object that has the `CreateStateMachine` trait implemented which can be done using the `#[derive(Behaviors)]` macro.


## Usage
You can find an example that uses both of these macros in the [arbiter-template repository](https://github.com/primitivefinance/arbiter-template). 
Similarly, in the Arbiter repo itself, this exact same collection of code is found in the `examples/template/` directory.

If you wanted to use the `#[main]` macro alongside the `#[derive(Behaviors)]` macro, you would do so like this:
```rust, ignore
use arbiter_macros::main;

use Behaviors; // From the Behaviors example above


#[main(
    name = "ExampleArbiterProject",
    about = "Our example to get you started.",
    behaviors = Behaviors
)]
pub async fn main() {}
```