# Arbiter

![](https://visitor-badge.laobi.icu/badge?page_id=arbiter)
![Github Actions](https://github.com/primitivefinance/arbiter/workflows/Rust/badge.svg)
[![](https://dcbadge.vercel.app/api/server/primitive?style=flat)](https://discord.gg/primitive)
[![Twitter Badge](https://badgen.net/badge/icon/twitter?icon=twitter&label)](https://twitter.com/primitivefi)

> Perform high speed modeling and economic fuzzing with EVM parity.

The Ethereum blockchain's execution environment, the Ethereum Virtual machine (EVM), contains a rich collection of decentralized applications. The EVM is stack machine that sequentially executes opcodes sent to it by users and smart contracts. Arbiter is a highly configurable rust interface over [revm](https://github.com/bluealloy/revm) which is a Rust implementation of the EVM stack machine logic. The purpose of Arbiter is to interface with arbitrary agents and contracts and run this all directly on a blazing-fast simulated EVM.

Financial engineers need to study a wide array of complex portfolio management strategies against thousands of market conditions, contract parameters, and agents. To configure such a rich simulation environment on a test network could be possible, but a more efficient choice for getting the most robust, yet quick, simulations would bypass any local networking and use a low level language's  implementation of the EVM.

Arbiter is being primarily developed to be a tool in evaluating economic and game theoretic security of DeFi applications. 

Arbiter can be used for:
- Evaluating the game theoretic and composable security of smart contracts in production environments (security firms and academics)
- investigating risk, capital efficiency, rebalancing strategies, and portfolio replication (or performance). (LPs, funds, quants, traders)
- Engineering and testing new financial products built on top of more primitive financial products (DeFi firms and academics)

## Features:

For our next beta release, we will be focusing on the following features:

#### TODOs:

- [x] Interface over Loading contracts with specific state
- [x] Interface over calldata
- [x] Interface over deploying contracts to REVM
- [ ] Case study on results of simulations.
- [ ] Documentation for the project.
- [ ] Publish to crates.io.

## Build From Source

First, clone the repository to your local environment so

```
git clone https://github.com/primitivefinance/arbiter.git
cd arbiter
cargo build
```

With the `arbiter` binary generated, you can run commands such as:
```
arbiter sim
```

## Generating Docs
To see the documentation for Arbiter, after cloning the repo, you can run:
```
cargo doc --workspace --no-deps --open
```
This will generate and open the docs in your browser. From there, you can look at the documentation for each crate in the Arbiter workspace.

## Including More Contracts

In the `lib/` folder you can add additional smart contracts or regenerate Rust bindings. To do so, run the following from the Arbiter root directory:

```
fforge bind -b crates/bindings/ --crate-name bindings --overwrite
```

## Contributing

See our [Contributing Guidelines](https://github.com/primitivefinance/arbiter/blob/main/.github/CONTRIBUTING.md)
