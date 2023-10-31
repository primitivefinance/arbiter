<img width="529" alt="arbiter" src="https://user-images.githubusercontent.com/20118821/236929861-2a1fe071-0053-453c-ac86-224b32febcd6.png">

> Expanding the EVM tooling ecosystem.

![Github Actions](https://github.com/primitivefinance/arbiter/workflows/test/badge.svg)
[![Codecov badge](https://codecov.io/gh/primitivefinance/arbiter/branch/main/graph/badge.svg?token=UQ1SE0D9IN)](https://codecov.io/gh/primitivefinance/arbiter)
![Visitors badge](https://visitor-badge.laobi.icu/badge?page_id=arbiter)
![Telegram badge](https://img.shields.io/endpoint?color=neon&logo=telegram&label=chat&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Farbiter_rs)
[![Discord badge](https://dcbadge.vercel.app/api/server/primitive?style=flat)](https://discord.gg/primitive)
[![Twitter Badge](https://badgen.net/badge/icon/twitter?icon=twitter&label)](https://twitter.com/primitivefi)

**Arbiter** is a framework for stateful Ethereum smart-contract simulation. 
The framework features an [`ethers-rs`](https://github.com/gakonst/ethers-rs) middleware built on top of [revm](https://github.com/bluealloy/revm) which allows the end user to interact with a sandboxed `revm` instance as if it were an Ethereum node. 
This provides a familiar interface for interacting with the Ethereum Virtual Machine (EVM), but with unrivaled speed. 
Furthermore, Arbiter provides containment and management for simulations. For a running list of vulnerabilities found with Arbiter, please see the [Vulnerability Corpu](vulnerability_corpus.md)

## Overview

The Arbiter workspace has three crates:
- `arbiter`: The binary crate that exposes a command line interface for initializing simulations via a templated repository and generating contract bindings needed for the simulation.
- `arbiter-core`: The lib crate that contains the core logic for the Arbiter framework including the `RevmMiddleware` discussed before, the `Environment` which envelopes simulations, and the `Manager` who controls a collection of environments.
- `arbiter-derive`: The lib crate that contains custom derive macros for more succinct simulation building.

The purpose of Arbiter is to provide a toolset to construct arbitrary agents (defined in Rust, by smart contracts, or even other FFI) and have these agents interact with an Ethereum-like environment of your design. 
All contract bytecode is run directly using a blazing-fast EVM instance `revm` (which is used in live RPC nodes such as [`reth`](https://github.com/paradigmxyz/reth)) so that your contracts are tested in the exact same type of environment that they are deployed in.

## Motivation 

Smart contract engineers need to test their contracts against a wide array of potentially adversarial environments and contract parameters. 
The static stateless testing of contracts can only take you so far. 
To truly test the security of a contract, you need to test it against a wide array of dynamic environments that encompass the externalities of Ethereum mainnet. 
We wanted to do just that with Arbiter. 

Both smart contract and financial engineers come together in Decentralized Finance (DeFi) to build and deploy a wide array of complex decentralized applications as well as financial strategies respectively. 
For the latter, a financial engineer may want to test their strategies against thousands of market conditions, contract settings, shocks, and autonomous or random or even AI agents all while making sure their strategy isn't vulnerable to bytecode-level exploits.

To configure such a rich simulation environment on a test or local network is also possible with Arbiter by a change in choice of middleware. 
The most efficient choice for getting robust, yet quick, simulations would bypass any networking and use a low-level language's implementation of the EVM. 
Furthermore, we can gain control over the EVM world-state by working directly on `revm`.
We would like the user to have a choice in how they want to simulate their contracts and Arbiter provides that choice.

### Sim Driven Development and Strategization 

Test-driven development is a popular engineering practice to write tests first, which fail, and implement logic to get the test to eventually pass. 
With simulation-driven development, it's possible to build "tests" that can only pass if the *incentives* actually work. For example, a sim driven test might be `is_loan_liquidated`, and a simulation must be made for a liquidator agent to do the liquidation. 
This approach significantly improves the testing of economic systems and other mechanism designs, which is important in the world of networks that are mostly incentive-driven.

The same goes for developing strategies that one would like to deploy on a live Ethereum network. 
One can use Arbiter to simulate their strategy with an intended goal and see if it actually works. 
This is especially important in the world of DeFi where strategies are often a mix of on and off-chains and are susceptible to exploits.

## Installation

To install Arbiter, you will need to have Rust installed on your machine. 
You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install). 
Once you have Rust installed, you can install Arbiter by running the following commands:

```bash
cargo install arbiter
```
This will install the Arbiter binary on your machine. You can then run `arbiter --help` to see that Arbiter was installed properly as well as see the help menu.

## Command Line Interface 

The Arbiter binary provides a CLI for creating new project much like [Foundry](https://github.com/foundry-rs/foundry), which Arbiter aims to work alongside with. 
It also gives you the ability to fork a state of an EVM network and store it on disk so that you can use this fork in a simulation.

### Initialization 
To create a new project, you should have Foundry installed.
You can find the installation [here](https://getfoundry.sh/). 
To create a new Arbiter project, you can run:

```bash
arbiter init your-project-name
cd your-project-name
```

This initializes a new Arbiter project with a template. You can generate the bindings again by running:

```bash
arbiter bind
```

The template is executable at this point and you can run it by running:
```bash
cargo run
```

**Optional Arguments**

You can run `arbiter init <simulation_name> --no-git` to remove the `.git` directory from the template upon initialization.


### Bindings
You can load or write your own smart contracts in the templates `contracts/` directory and begin writing your own simulations. 
Arbiter treats Rust smart-contract bindings as first-class citizens. The contract bindings are generated via Foundry's `forge` command. 
`arbiter bind` wraps `forge` with some convenience features that will generate all your bindings to src/bindings as a rust module. 
[Foundry](https://github.com/foundry-rs/foundry) power-users are welcome to use `forge` directly.


### Forking

To fork a state of an EVM network, you must first create a fork config file.
An example is provided in the `example_fork` directory.
Essentially, you provide your storage location for the data, the network you want the block number you want, and metadata about the contracts you want to fork.

```bash
arbiter fork <fork_config.toml>
```

This will create a fork of the network you specified in the config file and store it in the location you specified.
It can then be loaded into an `arbiter-core` `Environment` by using the `Fork::from_disk()` method.

Forking is done this way to make sure that all emulation done does not require a constant connection to an RPC-endpoint.

**Optional Arguments** 
You can run `arbiter fork <fork_config.toml> --overwrite` to overwrite the fork if it already exists.


### Optional Arguments

You can run `arbiter init <simulation_name> --no-git` to remove the `.git` directory from the template upon initialization.


## Documentation

To see the documentation for the Arbiter crates, please visit the following:
- [`arbiter`](https://docs.rs/crate/arbiter/0.3.2/)
- [`arbiter-core`](https://docs.rs/arbiter-core/0.5.1/arbiter_core/)
- [`arbiter-derive`](https://docs.rs/arbiter-derive/0.1.0/arbiter_derive/)

You will also find each of these on crates.io.

## Benchmarks

In `arbiter-core`, we have a small benchmarking suite that compares the `RevmMiddleware` implementation over the `Environment` to the [Anvil](https://github.com/foundry-rs/foundry/tree/master/crates/anvil) local testnet chain implementation.
The biggest reasons why we chose to build Arbiter were to gain more control over the EVM environment and to have a more robust simulation framework, but we also wanted to gain in speed which is why we chose to build our own interface over `revm` as opposed to using Anvil (which does use `revm` under the hood). 
For the following, Anvil was set to mine blocks for each transaction as opposed to setting an enforced block time, and the `Environment` was set with a block rate of 10.0 (this was chosen somewhat arbitrarily as we will add in more block control in the future).
Preliminary benchmarks of the `RevmMiddleware` interface over `revm` against Anvil are given in the following table.

| Operation       |  RevmMiddlwware |    Anvil     | Relative Difference |
|-----------------|-----------------|--------------|---------------------|
| Deploy          | 241.819µs       | 8.215446ms   | ~33.97x             |
| Lookup          | 480.319µs       | 13.052063ms  | ~27.17x             |
| Stateless Call  | 4.03235ms       | 10.238771ms  | ~2.53x              |
| Stateful Call   | 843.296µs       | 153.102478ms | ~181.55x            |

The above can be described by:
- Deploy: Deploying a contract to the EVM. 
We deployed both `ArbiterToken` and `ArbiterMath` in this method, so you can divide the time by two to get an estimate of the time it takes to deploy a single contract.

- Lookup: Looking up at the `balanceOf` for a client's address for `ArbiterToken`.
We called `ArbiterToken`'s `balanceOf` function 100 times in this method.
Divide by 100 to get the time it takes to lookup a single balance.

- Stateless Call: Calling a contract that does not mutate state. 
We called `ArbiterMath`'s `cdf` function 100 times in this method.
Divide by 100 to get the time it takes to call a single stateless function.

- Stateful Call: Calling a contract that mutates state. 
We called `ArbiterToken`'s `mint` function 100 times in this call.
Divide by 100 to get the time it takes to call a single stateful function.

The benchmarking code can be found in the `arbiter-core/benches/` directory and these specific times were achieved over a 1000 run average. 
The above was achieved by running `cargo bench --package arbiter-core` which will automatically run with the release profile.
Times were achieved on an Apple Macbook Pro M1 Max with 8 performance and 2 efficiency cores and with 32GB of RAM.

Of course, the use cases of Anvil and the `RevmMiddleware` can be different. 
Anvil represents a more realistic environment with networking and mining, while the `RevmMiddleware` is a simpler environment with the bare essentials to running stateful simulations.
Anvil also mines blocks for each transaction, while the `RevmMiddleware` does not.
We hope to improve our API to allow the end user to be able to interface with their own choice of EVM environment to suit whatever their needs may be!

Please let us know if you find any issues with these benchmarks or if you have any suggestions on how to improve them!

## Testing

If you contribute please write tests for any new code you write, To run the tests, you can run:

```bash
cargo test --all --all-features
```

## Contributing

See our [Contributing Guidelines](https://github.com/primitivefinance/arbiter/blob/main/.github/CONTRIBUTING.md)
