# Arbiter (In Progress)

![](https://visitor-badge.laobi.icu/badge?page_id=arbiter)

> A stand-alone Rust program to events on UniswapV3 pools and...

## Motivation:

Arbitrage takes advantage of price discrepancies between two (or more) different exchanges. Right now, searchers can detect differences in reported prices from DEXs and execute pure profit trades, which we call *atomic arbitrage*. The environment for arbitrage is competitive, but for less popular token pairs, there is room for new searchers to bring prices into equilibrium amongst exchanges. 

We want to develop a Rust program capable of detecting and executing arbitrage opportunities between exchanges. This will be provided to the community to lower the entry barrier into searching and thus level the playing field a bit more in the arbitrage scene.

## Features (in development):
 
#### TODOs: 
- [x] Library of popular tokens includes symbols, addresses, and decimals. 
- [x] Takes user input of token addresses and finds the corresponding PoolIDs for UniswapV3.
- [x] Monitors the UniswapV3 pool prices continuously.
- [ ] Allow users to input an Etherscan API key to pull token info from the chain.
- [ ] Integrate with user-selected RPC endpoint.
- [ ] Concurrent pool monitoring for multiple pools.
- [ ] Additional exchanges.
- [ ] Announces when an arbitrage trade with profit exceeding the no-arbitrage bounds + gas cost is found between two exchanges.
- [ ] Executes atomic transactions between pools to capture arbitrage.

## Build From Source

First, clone the repository to your local environment so
```
git clone https://github.com/primitivefinance/arbiter.git
cd arbiter
```

`arbiter` takes in three command line arguments. To see the available arguments, run the following:
```
cargo run -- -h
```
This will display the `help` menu
```console
arbiter 0.0.1

USAGE:
    arbiter [FLAGS] [OPTIONS]

FLAGS:
        --api_key    Provide an Etherscan API key if function calls require it (e.g., providing a token that is not in
                     the database).
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --fee <fee>          Specifies the basis points for the pool. [1, 5, 30, 100] [default: 5]
        --token0 <token0>    Specifies the first token for a token pair. [default: ETH]
        --token1 <token1>    Specifies the second token for a token pair which will be the numerier. [default: USDC]
```

In the above, `token0` and `token1` will be the token pair used to find the corresponding UniswapV3 pools. Upon running
```
cargo run
```
we default to `token0=ETH`, `token1=USDC`, and `fee=5`. `arbiter` will return results
```console
Uniswap Pool Result: Uniswap Pool Result: 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640
```
which is the [5BP pool](https://info.uniswap.org/#/pools/0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640) for the pair ETH/USDC on Uniswap. The program runs and streams transactions (swaps) that update the pool's price like so:
``` console
------------New Swap------------
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
cargo run -- --token0 USDC --token1 ETH --fee 30
```
Which will return the pool address and then log swaps on this pool with the price now denominated in ETH
``` console
Uniswap Pool Result: 0xe0554a476a092703abdb3ef35c80e0d76d32939f
------------New Swap------------
From pool 0xe0554a476a092703abdb3ef35c80e0d76d32939f
Sender: 0x1d64fb0ffa8362b2e1ee7ee03929159551eab26e, Recipient: 0x76f4eed9fe41262669d0250b2a97db79712ad855
amount_0 -1087770096
amount_1 900000000000000000
liquidity 232281109704283752
tick 205348
price "8.274094178028810096953876094149600409854e+20"
```
You may also build the executable with `cargo build`, which will output a binary in `target/`

## Contributing

If you want to see a specific feature, open an issue!
See our [Contributing Guidlines](https://github.com/primitivefinance/.github/pull/3/files#diff-eca12c0a30e25b4b46522ebf89465a03ba72a03f540796c979137931d8f92055)
