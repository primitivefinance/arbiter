# Arbiter
**Arbiter** is a framework for stateful Ethereum smart-contract simulation. 
The framework features an [`ethers-rs`](https://github.com/gakonst/ethers-rs) middleware built on top of [revm](https://github.com/bluealloy/revm) which allows the end user to interact with a sandboxed `revm` instance as if it were an Ethereum node. 
This provides a familiar interface for interacting with the Ethereum Virtual Machine (EVM), but with unrivaled speed. 
Furthermore, Arbiter provides containment and management for simulations. For a running list of vulnerabilities found with Arbiter, please see the [Vulnerability Corpus](./documentation/src/contributing/vulnerability_corpus.md).

## Overview
The Arbiter workspace has three crates:
- `arbiter`: The binary crate that exposes a command line interface for initializing simulations via a templated repository and generating contract bindings needed for the simulation.
- `arbiter-core`: The lib crate that contains the core logic for the Arbiter framework including the `RevmMiddleware` discussed before, the `Environment` which envelopes simulations, and the `Manager` who controls a collection of environments.
- `arbiter-engine`: The lib crate that provides abstractions for building simulations and more.

The purpose of Arbiter is to provide a toolset to construct arbitrary agents (defined in Rust, by smart contracts, or even other foreign function interfaces) and have these agents interact with an Ethereum-like environment of your design. 
All contract bytecode is run directly using a blazing-fast EVM instance `revm` (which is used in live RPC nodes such as [`reth`](https://github.com/paradigmxyz/reth)) so that your contracts are tested in the exact same type of environment that they are deployed in.

## Motivation 
Smart contract engineers need to test their contracts against a wide array of potentially adversarial environments and contract parameters. 
The static stateless testing of contracts can only take you so far. 
To truly test the security of a contract, you need to test it against a wide array of dynamic environments that encompass the externalities of Ethereum mainnet. 
We wanted to do just that with Arbiter. 

Both smart contract and financial engineers come together in Decentralized Finance (DeFi) to build and deploy a wide array of complex decentralized applications as well as financial strategies respectively. 
For the latter, a financial engineer may want to test their strategies against thousands of market conditions, contract settings, shocks, and autonomous or random or even AI agents all while making sure their strategy isn't vulnerable to bytecode-level exploits.

To configure such a rich simulation environment on a test or local network is also possible with Arbiter by a change in choice of middleware. 
The most efficient choice for getting robust, yet quick, simulations would bypass any networking and use a low level language's implementation of the EVM. 
Furthermore, we can gain control over the EVM worldstate by working directly on `revm`.
We would like the user to have a choice in how they want to simulate their contracts and Arbiter provides that choice.

### Sim Driven Development and Strategization 

Test driven development is a popular engineering practice to write tests first, which fail, and implement logic to get the test to eventually pass. 
With simulation driven development, it's possible to build "tests" that can only pass if the *incentives* actually work. For example, a sim driven test might be `is_loan_liquidated`, and a simulation must be made for a liquidator agent to do the liquidation. 
This approach significantly improves the testing of economic systems and other mechanism designs, which is important in the world of networks that are mostly incentive driven.

The same goes with developing strategies that one would like to deploy on a live Ethereum network. 
One can use Arbiter to simulate their strategy with an intended goal and see if it actually works. 
This is especially important in the world of DeFi where strategies are often a mix of on and offchain and are susceptible to exploits.

## Developer Documentation
To see the documentation for the Arbiter crates, please visit the following:
- [`arbiter`](https://docs.rs/crate/arbiter/)
- [`arbiter-bindings`](https://docs.rs/crate/arbiter-bindings/)
- [`arbiter-core`](https://docs.rs/arbiter-core/)

You will also find each of these on crates.io.

## Contributing

See our [Contributing Guidelines](https://github.com/primitivefinance/arbiter/blob/main/.github/CONTRIBUTING.md)
