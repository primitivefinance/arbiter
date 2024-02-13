pub mod behaviors;
pub mod bindings;

use behaviors::Behaviors;

/// To run this example, you can do the following from the `arbiter/` directory:
/// ```sh
/// cargo run --example project simulate examples/project/configs/example.toml
/// ```
/// If you would like to see more detailed logs, you can run the following:
/// ```sh
/// cargo run --example project simulate examples/project/configs/example.toml -vvv
/// ```
/// to get `debug` level logs.
///
/// By running
/// ```sh
/// cargo run --example project
/// ```
/// you will get the `--help` message for the project.
#[arbiter_macros::main(
    name = "ExampleArbiterProject",
    about = "Our example to get you started.",
    behaviors = Behaviors
)]
pub async fn main() {}
