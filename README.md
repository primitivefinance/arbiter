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



The Ethereum blockchain's execution environment, the Ethereum Virtual machine (EVM), contains a rich collection of decentralized applications. The EVM is stack machine that sequentially executes opcodes sent to it by users and smart contracts. Arbiter is a highly configurable rust interface over [revm](https://github.com/bluealloy/revm) which is a Rust implementation of the EVM stack machine logic. The purpose of Arbiter is to interface with arbitrary agents and contracts and run this all directly on a blazing-fast simulated EVM.

Financial engineers need to study a wide array of complex portfolio management strategies against thousands of market conditions, contract parameters, and agents. To configure such a rich simulation environment on a test network could be possible, but a more efficient choice for getting the most robust, yet quick, simulations would bypass any local networking and use a low level language's implementation of the EVM.

Arbiter is being primarily developed to be a tool in evaluating economic and game theoretic security of DeFi applications.

Arbiter can be used for:

- Evaluating the game theoretic and composable security of smart contracts in production environments (security firms and academics)
- investigating risk, capital efficiency, rebalancing strategies, and portfolio replication (or performance). (LPs, funds, quants, traders)
- Engineering and testing new financial products built on top of more primitive financial products (DeFi firms and academics)

## Documentation

## Installation

### Build from source

```bash
git clone https://github.com/primitivefinance/arbiter.git
cd arbiter
cargo install --path .
```

## CLI 

```bash
arbiter init your-project-name
cd your-project-name
arbiter bind
cargo run
```


## Generating Docs

To see the documentation for Arbiter, after cloning the repo, you can run:

```bash
cargo doc --workspace --no-deps --open
```

This will generate and open the docs in your browser. From there, you can look at the documentation for each crate in the Arbiter workspace. 
We will post both crates to crates.io once we have removed any and all Github linked crates.

## Contributing

See our [Contributing Guidelines](https://github.com/primitivefinance/arbiter/blob/main/.github/CONTRIBUTING.md)
