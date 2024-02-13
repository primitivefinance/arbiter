# Examples

We have a few examples to help you get started with Arbiter. These examples are designed to be simple and easy to understand. They are also designed to be easy to run and modify. We hope you find them helpful!

Our examples are in the [examples](https://github.com/primitivefinance/arbiter/tree/main/examples) directory. There are two examples: one for building a simulation and one fork forking the mainnet state.

## Simulation

You can run them with the following command:

```bash
cargo run --example project simulate examples/project/configs/example.toml
```

This will run the minimal counter-simulation. The simulation is very minimal and is designed to be easy to understand. It uses an arbiter main macro to derive the `incrementer` behavior for a single agent. Our design philosophy is that the users of Arbiter should only need to define behaviors and a configuration toml for the behaviors. You can see how the behaviors were represented in this simulation in the [behaviors](https://github.com/primitivefinance/arbiter/tree/main/examples/project/behaviors) module. We implement a single behavior for the incrementer struct that deploys the counter on startup and then on the increment event will increment the count. 

For more information on the behavior trait please see the section on [behaviors](https://primitivefinance.github.io/arbiter/usage/arbiter_engine/behaviors.html)


## Forking

You can run the fork example with the following command:

```bash
arbiter fork examples/fork/weth_config.toml
```

This will fork the state specified in the `weth_config.toml` file. If you would like to fork a different state, you can modify the `weth_config.toml` file to point to include additional EOAs or contract storage. Once you have forked the state you want, you can start your simulation with the forked state by loading it into a memory revm instance like so:

```rust ignore
use arbiter_core::{database::Fork::*, Environment, ArbiterMiddleware};

let fork = Fork::from_disk("tests/fork.json").unwrap();

// Get the environment going
let environment = Environment::builder().with_db(fork.db).build();

// Create a client
let client = ArbiterMiddleware::new(&environment, Some("name")).unwrap();
```


