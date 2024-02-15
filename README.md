<img width="529" alt="arbiter" src="https://user-images.githubusercontent.com/20118821/236929861-2a1fe071-0053-453c-ac86-224b32febcd6.png">

> Expanding the EVM tooling ecosystem.

![Github Actions](https://github.com/primitivefinance/arbiter/workflows/test/badge.svg)
![Visitors badge](https://visitor-badge.laobi.icu/badge?page_id=arbiter)
![Telegram badge](https://img.shields.io/endpoint?color=neon&logo=telegram&label=chat&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Farbiter_rs)
[![Twitter Badge](https://badgen.net/badge/icon/twitter?icon=twitter&label)](https://twitter.com/primitivefi)

## Overview
**Arbiter** is a blazing-fast Ethereum sandbox to let developers orchestrate event-driven simulations.
The framework allows for fine grained control over an (Rust) Ethereum Virtual Machine (EVM) to provide stateful Ethereum smart-contract interactions and the creation of behaviors that can be coalesced into complex scenarios or automations. 
We use [`ethers-rs`](https://github.com/gakonst/ethers-rs) middleware on top of [revm](https://github.com/bluealloy/revm), which is used in ETH clients such as [`reth`](https://github.com/paradigmxyz/reth) as well as [`foundry`](https://github.com/foundry-rs/foundry).
This gives us speed, configurability, and modularity that feels like a light-weight custom Ethereum node. 

A main use of Arbiter is to probe mechanism security of smart contracts.
If this interests you, please see the [Vulnerability Corpus](https://primitivefinance.github.io/arbiter/vulnerability_corpus.html).

---

The Arbiter workspace has five crates:
- `arbiter`: The bin that exposes a command line interface for forking and binding contracts.
- `arbiter-core`: A lib that contains the core logic for the Arbiter framework including the `ArbiterMiddleware` discussed before, and the `Environment` which is our sandbox.
- `arbiter-engine`: A lib that provides abstractions for building simulations, agents, and behaviors.
- `arbiter-macros`: A lib crate that contains the macros used to simplify development with Arbiter.
- `arbiter-bindings`: A lib crate that contains bindings for utility smart contracts that are used for testing and development.


## Book
Here you can find the [Arbiter Documentation](https://primitivefinance.github.io/arbiter/).
This is an mdbook that provides higher level understanding of how to use the entirety of the Arbiter framework.

## Motivation 
Arbiter was built to provide you with the abilities to work with smart contracts in a stateful sandbox and design agents that can be used alongside the contracts.
This gives you many capabilities.
For instance, smart contract engineers need to test their contracts against a wide array of potentially adversarial environments and parameters, and not just rely on static stateless testing. 

In Decentralized Finance (DeFi), there are a wide array of complex decentralized applications that can use the testing described above, but there are also the implicit financial strategies that encompass many agents and parameterizations. 
A financial engineer may want to test their strategies against thousands of market conditions, contract settings, shocks, and autonomous or random or even AI agents all while making sure their strategy isn't vulnerable to bytecode-level exploits.
Likewise, the same engineer may want to develop searcher agents, solver agents, or other automatous agents that can be run onchain.

### Sim Driven Development

Test driven development is a popular engineering practice to write tests first, which fail, and implement logic to get the test to eventually pass. 
With Arbiter you can do simulation driven development.
It's possible to build "tests" that can only pass if the *incentives* actually work. 
For example, a sim driven test might be `is_loan_liquidated`, and a simulation must be made for a liquidator agent to do the liquidation. 
This approach significantly improves the testing of economic systems and other mechanism designs, which is important in the world of networks that are mostly incentive driven.

## Working with the Arbiter Framework
To work with Arbiter, you will need to have Rust installed on your machine. 
You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install). 
It will also be helpful to get the `cargo-generate` package which you can install by doing:
```bash
cargo install cargo-generate
```

### Examples
We have an example that will run what we also have set up in a template.
To run this, you can clone the repository and update the submodules:
```bash
git clone https://github.com/primitivefinance/arbiter.git
cd arbiter
git submodule update --init --recursive
```
From here, you can now try running the following from the clone's root directory:
```bash
cargo run --example template 
```
This command will enter into the template CLI and show you the set of commands and flags.

To run the `ModifiedCounter.sol` example and see some logging, try:
```bash
cargo run --example template simulate examples/template/configs/example.toml -vvv
```
This sets the log level to `debug` so you can see what's happening internally.

### Initialization
To create your own Arbiter project off of our template [arbiter-template](https://github.com/primitivefinance/arbiter-template) you can run the following:
```bash
cd <your/chosen/directory>
cargo generate https://github.com/primitivefinance/arbiter-template.git
```
You'll be prompted to provide a project name, and the rest will be set up for you!

### Binary
To install the Arbiter binary, run:
```bash
cargo install arbiter
```
This will install the Arbiter binary on your machine. You can then run `arbiter --help` to see that Arbiter was installed properly as well as see the help menu.

### Bindings
You can load or write your own smart contracts in the `contracts/` directory of your templated project and begin writing your own simulations. 
Arbiter treats Rust smart-contract bindings as first-class citizens. 
The contract bindings are generated via Foundry's `forge` command. 
`arbiter bind` wraps `forge` with some convenience features that will generate all your bindings to `src/bindings` as a Rust module. 
[Foundry](https://github.com/foundry-rs/foundry) power-users are welcome to use `forge` directly.

### Forking
To fork a state of an EVM network, you must first create a fork config file.
An example is provided in the `examples/fork` directory.
Essentially, you provide your storage location for the data, the network you want the block number you want, and metadata about the contracts you want to fork.

```bash
arbiter fork <fork_config.toml>
```
This will create a fork of the network you specified in the config file and store it in the location you specified.
It can then be loaded into an `arbiter-core` `Environment` by using the `Fork::from_disk()` method.

Forking is done this way to make sure that all emulation done does not require a constant connection to an RPC-endpoint.
You may find that [Anvil](https://book.getfoundry.sh/anvil/) has an easier forking interface to work with, however, it is an online forking mechanism which makes RPC calls to update state as necessary.
Arbiter `Environment` forking is for creating state, storing it locally, and being able to initialize a simulation from that state when desired.
We do plan to allow `arbiter-engine` to integrate with other network types such as Anvil in the future!

**Optional Arguments** 
You can run `arbiter fork <fork_config.toml> --overwrite` to overwrite the fork if it already exists.

## Cargo Docs

To see the Cargo docs for the Arbiter crates, please visit the following:
- [`arbiter`](https://docs.rs/crate/arbiter/)
- [`arbiter-bindings`](https://docs.rs/crate/arbiter-bindings/)
- [`arbiter-core`](https://docs.rs/arbiter-core/)
- [`arbiter-macros`](https://docs.rs/arbiter-macros/)
- [`arbiter-engine`](https://docs.rs/arbiter-engine/)

You will find each of these on crates.io.

## Benchmarks
In `arbiter-core`, we have a a small benchmarking suite that compares the `ArbiterMiddleware` implementation over the `Environment` to the [Anvil](https://github.com/foundry-rs/foundry/tree/master/crates/anvil) local testnet chain implementation.
The biggest reasons why we chose to build Arbiter was to gain more control over the EVM environment and to have a more robust simulation framework, but we also wanted to gain in speed which is why we chose to build our own interface over `revm` as opposed to using Anvil (which does use `revm` under the hood). 
For the following, Anvil was set to mine blocks for each transaction as opposed to setting an enforced block time and the `Environment` was set with a block rate of 10.0 (this was chosen somewhat arbitrarily as we will add in more block control in the future).
Preliminary benchmarks of the `ArbiterMiddleware` interface over `revm` against Anvil are given in the following table.

to run the benchmarking code yourself, you can run:
```bash
cargo bench --package arbiter-core
```

| Operation       |  ArbiterMiddleware |    Anvil     | Relative Difference |
|-----------------|-----------------|--------------|---------------------|
| Deploy          | 238.975µs       | 7712.436µs   | ~32.2729x           |
| Lookup          | 565.617µs       | 17880.124µs  | ~31.6117x           |
| Stateless Call  | 1402.524µs      | 10397.55µs   | ~7.413456x          |
| Stateful Call   | 2043.88µs       | 154553.225µs | ~75.61756x          |


The above can be described by:
- Deploy: Deploying a contract to the EVM. 
We deployed both `ArbiterToken` and `ArbiterMath` in this method, so you can divide the time by two to get an estimate for the time it takes to deploy a single contract.

- Lookup: Looking up a the `balanceOf` for a client's address for `ArbiterToken`.
We called `ArbiterToken`'s `balanceOf` function 100 times in this method.
Divide by 100 to get the time it takes to lookup a single balance.

- Stateless Call: Calling a contract that does not mutate state. 
We called `ArbiterMath`'s `cdf` function 100 times in this method.
Divide by 100 to get the time it takes to call a single stateless function.

- Stateful Call: Calling a contract that mutates state. 
We called `ArbiterToken`'s `mint` function 100 times in this call.
Divide by 100 to get the time it takes to call a single stateful function.

The benchmarking code can be found in the `arbiter-core/benches/` directory and these specific times were achieved over a 1000 run average. 
The above was achieved running `cargo bench --package arbiter-core` which will automatically run with the release profile.
Times were achieved on an Apple Macbook Pro M1 Max with 8 performance and 2 efficiency cores, and with 32GB of RAM.

Of course, the use cases of Anvil and the `ArbiterMiddleware` can be different. 
Anvil represents a more realistic environment with networking and mining, while the `ArbiterMiddleware` is simpler environment with the bare essentials to running stateful simulations.
Anvil also mines blocks for each transaction, while the `ArbiterMiddleware` does not.

Please let us know if you find any issues with these benchmarks or if you have any suggestions on how to improve them!

## Testing

If you contribute please write tests for any new code you write, To run the tests, you can run:

```bash
cargo test --all --all-features
```

## Contributing

See our [Contributing Guidelines](https://github.com/primitivefinance/arbiter/blob/main/.github/CONTRIBUTING.md)
