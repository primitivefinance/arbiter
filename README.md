# Arbiter (In Progress)

![](https://visitor-badge.laobi.icu/badge?page_id=arbiter)
![Github Actions](https://github.com/primitivefinance/arbiter/workflows/Rust/badge.svg) 
[![](https://dcbadge.vercel.app/api/server/primitive?style=flat)](https://discord.gg/primitive)
[![Twitter Badge](https://badgen.net/badge/icon/twitter?icon=twitter&label)](https://twitter.com/primitivefi)

> Code that runs code that runs code to study how code runs code and behaves in any environment.

Ethereum's execution environment, the Ethereum virtual machine (EVM), has given fruit to a rich collection of decentralized applications. The EVM resembles a simple stack machine and sequentially executes opcodes as decentralized applications are used, deployed, or exploited.

With the rich new array of applications and builders, there becomes an increasing need for developer tools. The developer tooling for the Ethereum ecosystem has increased drastically over the past years. For example, [Foundry](https://github.com/foundry-rs/foundry)'s integration of [revm](https://github.com/bluealloy/revm), a rust implementation of the EVM, allows them to fuzz smart contracts at high speeds. Revm allows for much more resilient testing and helps detect anomaly behavior. While we have come far, we still need critical testing infrastructure. Because the Ethereum production environment is open and available to anyone, it would be nice to see how a given smart contract would interact with existing smart contracts at scale. With the revm, we can achieve this. If we want to study arbitrage in a production environment so we can account for things as granular as gas, and we don't want to lose any precision (EVM types are u256 which not all other languages support the same quantity of accuracy natively), this isn't easy with current tools. If we upload the opcodes of these contracts and algorithmic calls from an arbitrage script to the revm, we can have thousands of data points in a matter of minutes. If we wanted to do this on a test network, we would be limited by the network's block times. For this case, many innovators and scientists have resulted in simulations with less granularity and precision. While this can help validate the theory, it tells us little about how these theories hold up in practice.


Arbiter implements revm (in Rust) to build highly configurable and efficient simulations. Since there is a 1-1 mapping of the revm and EVM opcodes, to switch to an execution environment, all you have to do is change your endpoint.

Financial engineers have the motive to study complex portfolio management strategies in significant numbers against many market conditions, contract parameters, and arbitrageurs. To configure such a rich simulation environment on a test network would take months to get a sufficient quantity of data points to draw conclusions with confidence and isolate ket variables. If we modeled this problem with historical data but omitted gas or a certain level of precision, our results would also be insignificant. But with REVM, we would be about to generate hundreds of thousands of data points (spanning years of data) in a matter of (estimated) hours or days. In financial engineering, this is a critical tool in evaluating capital efficiency, loss vs. rebalancing, and game theoretic security.

Thus arbiter as a tool will bring value to individuals and teams.

- Evaluating the game theoretic and composable security of smart contracts in production environments (Security Firms and Academics)
- Engineering and testing new financial products built on top of more primitive financial products (DeFi Firms and Academics)
- Evaluating financial risk and mitigation strategies (Funds, prop-shops, searchers)

With Arbiter, we aim to build a layer of abstraction to lower the required technical competence to support more study and experimentation. To get a head start, we built an open-source MVP intending to abstract away complexity at different layers of the blockchain stack and are using the uniswapV3 core contracts to test our implementation.

- Arbiter consists of three primary components:
    - **Architect:** A simple execution module to bundle and execute transactions (supports flashbot bundles).
    - **Clairvoyance:** A live monitoring tool supporting historical and real-time concurrent contract data monitoring.
    - **Simulate** toolbox for testing strategies.


## Architecture:

There are three primary crates: `architect` which handles the construction and execution of transactions, and `clairvoyance`, which handles the live monitoring of contract events, and `simulate`, which handles a large multi-agent simulations in revm. The program is designed to be modular and extensible.

## Features:

For our next beta release, we will be focusing on the following features:

#### TODOs:

- [x] Library of popular tokens includes symbols, addresses, and decimals.
- [x] Takes user input of token addresses and finds the corresponding PoolIDs for UniswapV3.
- [x] Monitors the UniswapV3 pool prices continuously.
- [x] Integrate with user-selected RPC endpoint.
- [x] Concurrent pool monitoring for multiple pools.
- [x] Data Monitoring Component: Clairvoyance.
- [x] Execution Component: Architect.
- [ ] Simulation component: Simulate.
- [ ] Simulate and measure arbitrage behavoir against different price paths on local test networks.
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
`arbiter` has many subcommands and therefore many arguments. To see the available arguments for pool monitoring, run the following:

```
arbiter see --help
```

This will display the `help` menu for the `Clairvoyance` crate

```console
Access the `Clairvoyance` monitoring module via this subcommand

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

## Test Network Simulations

In order to run a test network and simulate transactions with DEXs or other objects, we need `Foundry`. This can be run on your local machine, or an external machine. To get the full `Foundry` suite, run:

````
cargo install --git https://github.com/foundry-rs/foundry --profile local --locked foundry-cli anvil
```

From here, you need a way to fork the blockchain network you want. For example, you can create an Alchemy account, get an Alchemy API key. With this key, export it to your `env` variables by

```
export ALCHEMY_KEY = your_key_here
```

You can check this by `echo $ALCHEMY_KEY`. If all is well, you can run your fork of the Ethereum main-net by:

```
anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/
```

## Contributing

See our [Contributing Guidelines](https://github.com/primitivefinance/arbiter/blob/main/.github/CONTRIBUTING.md)
