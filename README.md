# Arbiter (In Progress)

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

## Event Monitoring

While `arbiter` is a powerful tool for modeling and fuzzing, it also supports event monitoring on uniswap V3 pools. To see the available arguments for pool monitoring, run the following:

```
arbiter see --help
```

This will display the `help` menu for this crate

```console
Usage: arbiter.exe see [TOKEN0] [TOKEN1] [BP] [CONFIG]

Arguments:
  [TOKEN0]  Token 0 of the pool [default: ETH]
  [TOKEN1]  Token 1 of the pool [default: USDC]
  [BP]      Basis point fee of the pool [default: 5]
  [CONFIG]    Sets a custom config file

Options:
  -h, --help  Print help information
```

In the above, `token0` and `token1` will be the token pair used to find the corresponding UniswapV3 pools. Upon running

```
arbiter see
```

we default to `token0=ETH`, `token1=USDC`, and `fee=5`. `arbiter` will return the [5BP pool](https://info.uniswap.org/#/pools/0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640) for the pair ETH/USDC on Uniswap. The program runs and streams transactions (swaps) that update the pool's price like so:

```console
------------NEW SWAP------------
From pool 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640
Sender: 0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45, Recipient: 0x1019bf2d607cc646a94a194f7a79e0b385065cff
amount_0 -5235133099
amount_1 4335000000000000000
liquidity 23260193077241608585
tick 205351
price "1.208239460504000000000000000000000000000e+3"
```

If you would like the price to be in terms of ETH rather than USD for a one basis point pool, you can run

```
arbiter see USDC ETH 30
```

Which will return the pool address and then log swaps on this pool with the price now denominated in ETH

```console
Uniswap Pool Result: 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640
------------NEW SWAP------------
Pool:      0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640
Sender:    0x1111111254fb6c44bac0bed2854e76f90643097d
Recipient: 0x134603117a253dd4550eb1fc508e289761be9c3e
Amount_0:  -1949252708
Amount_1:  1603051840877282447
Liquidity: 21972098821216706277
Tick:      205282
Price:     "1.216568804789000000000000000000000000000e+3"
```

The event monitoring feature will be depricated in a future release.

You may also build the executable with `cargo build`, which will output a binary in `target/`

## Configuration File

To run arbiter with a configuration file, we can run the command `cargo run see --config` or, if we have built the latest changes, `arbiter see --config`. This will load in configuration settings from a `config.toml` file located in `./crates/cli/src/`. Feel free to make custom configuration files for your needs. The logic that finds the file is in `crates/cli/src/config/mod.rs`; remember to add the path of your new file to the list of file paths.

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
