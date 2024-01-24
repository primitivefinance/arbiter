# Getting Started
To use Arbiter, you can use the Arbiter CLI to help you manage your projects or, if you feel you don't need any of the CLI features, you can be free to use the [`arbiter-core`](https://crates.io/crates/arbiter-core), `arbiter-engine`, and [`arbiter-bindings`](https://crates.io/crates/arbiter-bindings) crates directly.
You can find more information about these crates in the [Usage](../index.md) section.
The crates (aside from `arbiter-engine` at the moment) are linked to their crates.io pages so you can add them to your project by:
```toml
[dependencies]
arbiter-core = "*" # You can specify a version here if you'd like
arbiter-bindings = "*" # You can specify a version here if you'd like 
```

## CLI Prerequisites
Before installing Arbiter CLI, ensure that you have Rust installed. You can install and verify your Rust installation from the [official website](https://www.rust-lang.org/tools/install).

The Arbiter CLI works alongside [Foundry](https://github.com/foundry-rs/foundry) and aims to provide a similar CLI interface of setting up and interacting with Arbiter projects. 
Install Foundry from the [official website](https://getfoundry.sh/).