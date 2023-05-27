<img width="529" alt="arbiter" src="https://user-images.githubusercontent.com/20118821/236929861-2a1fe071-0053-453c-ac86-224b32febcd6.png">

> Expanding the EVM tooling ecosystem.

![Github Actions](https://github.com/primitivefinance/arbiter/workflows/test/badge.svg)
[![codecov](https://codecov.io/gh/funkycadet/arbiter/branch/main/graph/badge.svg?token=UQ1SE0D9IN)](https://codecov.io/gh/funkycadet/arbiter)
![](https://visitor-badge.laobi.icu/badge?page_id=arbiter)
[![](https://dcbadge.vercel.app/api/server/primitive?style=flat)](https://discord.gg/primitive)
[![Twitter Badge](https://badgen.net/badge/icon/twitter?icon=twitter&label)](https://twitter.com/primitivefi)

The Ethereum blockchain's execution environment, the Ethereum Virtual machine (EVM), contains a rich collection of decentralized applications. The EVM is stack machine that sequentially executes opcodes sent to it by users and smart contracts. Arbiter is a highly configurable rust interface over [revm](https://github.com/bluealloy/revm) which is a Rust implementation of the EVM stack machine logic. The purpose of Arbiter is to interface with arbitrary agents and contracts and run this all directly on a blazing-fast simulated EVM.

Financial engineers need to study a wide array of complex portfolio management strategies against thousands of market conditions, contract parameters, and agents. To configure such a rich simulation environment on a test network could be possible, but a more efficient choice for getting the most robust, yet quick, simulations would bypass any local networking and use a low level language's implementation of the EVM.

Arbiter is being primarily developed to be a tool in evaluating economic and game theoretic security of DeFi applications.

Arbiter can be used for:

- Evaluating the game theoretic and composable security of smart contracts in production environments (security firms and academics)
- investigating risk, capital efficiency, rebalancing strategies, and portfolio replication (or performance). (LPs, funds, quants, traders)
- Engineering and testing new financial products built on top of more primitive financial products (DeFi firms and academics)

## Features:

For our next beta release, we will be focusing on the following features:


## Build From Source

First, clone the repository to your local environment so

```bash
git clone https://github.com/primitivefinance/arbiter.git
cd arbiter
```

Install arbiter on your system:

```bash
cargo install --path . --force
```

With the `arbiter` binary generated, you can run commands such as:

```bash
arbiter simulate uniswap
```

## Generating Docs

To see the documentation for Arbiter, after cloning the repo, you can run:

```bash
cargo doc --workspace --no-deps --open
```

This will generate and open the docs in your browser. From there, you can look at the documentation for each crate in the Arbiter workspace.

## Including More Contracts

In the `contracts/` directory you can add additional smart contracts or regenerate Rust bindings. Once that is done, you will want to make sure the bindings are generated in the script:

```bash
./bind.sh
```
You will need to add the relevant directory for your new contracts to the script above and make sure they are also handled by `forge install`. We look forward to improving upon this UX in the future.

At the moment, this only builds the bindings for the contracts in the `lib/arbmod/contracts/` and `lib/portfolio/contracts`. You can of course add an additional directory of contracts in `lib/`. Just be sure to include it when you generate bindings!

## Contributing

See our [Contributing Guidelines](https://github.com/primitivefinance/arbiter/blob/main/.github/CONTRIBUTING.md)
