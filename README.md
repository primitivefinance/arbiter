# Arbiter

![](https://visitor-badge.laobi.icu/badge?page_id=arbiter)
![Github Actions](https://github.com/primitivefinance/arbiter/workflows/Rust/badge.svg)
[![](https://dcbadge.vercel.app/api/server/primitive?style=flat)](https://discord.gg/primitive)
[![Twitter Badge](https://badgen.net/badge/icon/twitter?icon=twitter&label)](https://twitter.com/primitivefi)

> Perform high speed modeling and economic fuzzing with EVM parity.

Ethereum's execution environment, the Ethereum virtual machine (EVM), has given fruit to a rich collection of decentralized applications. The EVM is stack machine that sequentially executes opcodes as decentralized applications are used, deployed, or exploited. Arbiter is a highly configurable rust interface over [revm](https://github.com/bluealloy/revm).

Financial engineers need to study complex portfolio management strategies against many market conditions, contract parameters, and agents. To configure such a rich simulation environment on a test network would take months to get a sufficient quantity of data points to draw conclusions with confidence and isolate key variables. Even with a local network with no block time, the networking latency on port to port communication will be significant.

In financial engineering, this is a critical tool in evaluating capital efficiency, loss vs. rebalancing, and game theoretic security. Arbiter can be used for:

- Evaluating the game theoretic and composable security of smart contracts in production environments (Security Firms and Academics)
- Engineering and testing new financial products built on top of more primitive financial products (DeFi Firms and Academics)
- Evaluating financial risk and mitigation strategies (Funds, prop-shops, searchers)

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
cargo install --path ./crates/cli
```

Set the PROVIDER environment variable to use a custom provider.

## Setting Custom RPC

If you would like to use your own RPC endpoint, then you can set the environment variable `PROVIDER`. By default, the provider we have set is via Alchemy. To set your own environment variable on a UNIX OS just perform:

```
export PROVIDER=https://url-to-your-RPC-endpoint.xyz
```

and replace your own URL as needed. Double check the environment variable is set by:

```
echo $PROVIDER
```

or just list all environment variables with:

```
env
```

If you need to unset the `PROVIDER` variable, do:

```
unset PROVIDER
```

## Contributing

See our [Contributing Guidelines](https://github.com/primitivefinance/arbiter/blob/main/.github/CONTRIBUTING.md)
