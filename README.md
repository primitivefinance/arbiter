<img width="529" alt="arbiter" src="https://user-images.githubusercontent.com/20118821/236929861-2a1fe071-0053-453c-ac86-224b32febcd6.png">

> Expanding the EVM tooling ecosystem.

![Github Actions](https://github.com/primitivefinance/arbiter/workflows/test/badge.svg)
[![Codecov badge](https://codecov.io/gh/funkycadet/arbiter/branch/main/graph/badge.svg?token=UQ1SE0D9IN)](https://codecov.io/gh/funkycadet/arbiter)
![Visitors badge](https://visitor-badge.laobi.icu/badge?page_id=arbiter)
![Telegram badge](https://img.shields.io/endpoint?color=neon&logo=telegram&label=chat&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Farbiter_rs)
[![Discord badge](https://dcbadge.vercel.app/api/server/primitive?style=flat)](https://discord.gg/primitive)
[![Twitter Badge](https://badgen.net/badge/icon/twitter?icon=twitter&label)](https://twitter.com/primitivefi)

**Arbiter** is a framework for stateful Ethereum smart-contract simulation. 
The framework features an [`ethers-rs`](https://github.com/gakonst/ethers-rs) middleware built on top of [revm](https://github.com/bluealloy/revm) which allows the end user to interact with a sandboxed `revm` instance as if it were an Ethereum node. This provides a familiar interface for interacting with the Ethereum Virtual Machine (EVM), but with unrivaled speed. Furthermore, Arbiter provides containment and management for simulations.

## Overview

The Arbiter workspace has two crates:
- `arbiter-core`: The lib crate that contains the core logic for the Arbiter framework including the `RevmMiddleware` discussed before, the `Environment` which envelopes simulations, and the `Manager` who controls a collection of environments.
- `arbiter`: The binary crate that exposes a command line interface for initializing simulations via a templated repository and generating contract bindings needed for the simulation.

The purpose of Arbiter is to provide a toolset to construct arbitrary agents (defined in Rust, by smart contracts, or even other FFI) and have these agents interact with an Ethereum-like environment of your design. 
All contract bytecode is run directly using a blazing-fast EVM instance `revm` (which is used in live RPC nodes such as [`reth`](https://github.com/paradigmxyz/reth)) so that your contracts are tested in the exact same type of environment that they are deployed in.

## Motivation 

Smart contract engineers need to test their contracts against a wide array of potentially adversarial environments and contract parameters. 
The static stateless testing of contracts can only take you so far. To truly test the security of a contract, you need to test it against a wide array of dynamic environments that encompass the externalities of Ethereum mainnet. We wanted to do just that with Arbiter. 

Both smart contract and financial engineers come together in Decentralized Finance (DeFi) to build and deploy a wide array of complex decentralized applications as well as fincancial strategies respectively. 
For the latter, a financial engineer may want to test their strategies against thousands of market conditions, contract settings, shocks, and autonomous or random or even AI agents all while making sure their strategy isn't vulnerable to bytecode-level exploits.

To configure such a rich simulation environment on a test or local network is also possible with Arbiter by a change in choice of middleware. 
The most efficient choice for getting robust, yet quick, simulations would bypass any networking and use a low level language's implementation of the EVM. 
Furthermore, we can gain control over the EVM worldstate by working directly on `revm`.
We would like the user to have a choice in how they want to simulate their contracts and Arbiter provides that choice.

### Sim Driven Development and Strategization 

Test driven development is a popular engineering practice to write tests first, which fail, and implement logic to get the test to eventually pass. With simulation driven development, it's possible to build "tests" that can only pass if the *incentives* actually work. For example, a sim driven test might be `is_loan_liquidated`, and a simulation must be made for a liquidator agent to do the liquidation. This approach significantly improves the testing of economic systems and other mechanism designs, which is important in the world of networks that are mostly incentive driven.

The same goes with developing strategies that one would like to deploy on a live Ethereum network. One can use Arbiter to simulate their strategy with an intended goal and see if it actually works. This is especially important in the world of DeFi where strategies are often a mix of on and offchain and are susceptible to exploits.

## Installation

To install Arbiter, you will need to have Rust installed on your machine. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install). Once you have Rust installed, you can install Arbiter by running the following commands:

```bash
git clone https://github.com/primitivefinance/arbiter.git
cargo install --path ./arbiter
```
This will install the Arbiter binary on your machine. You can then run `arbiter --help` to see that Arbiter was installed properly as well as see the help menu.

## Command Line Interface 

The Arbiter binary provides a CLI for creating new projects much like [Foundry](https://github.com/foundry-rs/foundry), which Arbiter aims to work alongside with. To create a new project, you can run:

```bash
arbiter init your-project-name
cd your-project-name
```

This initializes a new Arbiter project with a template. The template is executable at this point and you can run it by running:
```bash
cargo run
```
You can then load your own smart contracts into and begin writing your own simulations. Arbiter treats Rust smart-contract bindings as first-class citizens and provides a means to generate bindings of your own contracts (via Foundry's `forge` command). To generate bindings for your own contracts, you can run:

```bash
arbiter bind
```

This will generate bindings for all of the contracts in your `contracts` directory. You can then use these bindings in your simulations. 


## Documentation

To see the documentation for Arbiter, after cloning the repo, you can run:

```bash
cargo doc --workspace --no-deps --open
```

This will generate and open the docs in your browser. From there, you can look at the documentation for each crate in the Arbiter workspace. 
We will post both crates to crates.io once we have removed any and all Github linked crates.

## Contributing

See our [Contributing Guidelines](https://github.com/primitivefinance/arbiter/blob/main/.github/CONTRIBUTING.md)
