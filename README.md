# Arbiter (In Progress)

> A stand-alone Rust program to events on UniswapV3 pools and ...

## Motivation:

Arbitrage takes advantage of price discrepencies between two (or more) different exchanges. Right now, searchers are able to detect differences in reported prices from DEXs and execute pure profit trades which we call *atomic arbitrage*. The environment for arbitrage is competitive, but for less popular token pairs, there is room for new searchers to bring prices into equilibrium amongst exchanges. 

We want to develop a Rust program that is capable of detecting and executing on abritrage opprtunities between different exchanges. This will be provided to the community in order to lower the barrier of entry into searching and thus level the playing field a bit more in the arbitrage scene.

## Features (in development):
 
#### TODOs: 
- [x] Library of top 10 popular tokens which includes symbol, address, and decimals. 
- [x] Takes user input of token addresses and finds the corresponding PoolIDs for UniswapV3.
- [x] Monitors the UniswapV3 pool prices continuously.
- [ ] Allow for users to input an Etherscan API key to pull token info from the chain.
- [ ] Integrate with user selected RPC endpoint.
- [ ] Cuncurrent pool monitoring for multiple pools.
- [ ] Additional exchanges.
- [ ] Announces when an arbitrage trade with profit exceeding the no-arbitrage bounds + gas cost is found between two exchanges.
- [ ] Executes atomic transactions between pools to capture arbitrage.

## Use: Binary:

`arbiter` takes in three command line arguments. To see the available arguments, run:
```console
$  /arbiter -h
```
This will display the `help` menu
```console
arbiter 0.0.1

USAGE:
    app [FLAGS] [OPTIONS]

FLAGS:
        --api_key    Provide an Etherscan API key if function calls require it (e.g., providing a token that is not in
                     the database).
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --fee <fee>          Specifies the basis points for the pool. [1, 5, 30, 100] [default: 5]
        --token0 <token0>    Specifies the first token for a token pair. [default: ETH]
        --token1 <token1>    Specifies the second token for a token pair which we be the numerier. [default: USDC]
```

In the above, `token0` and `token1` will be the token pair used to find the corresponding UniswapV3 pools. Upon running
```console
$ cargo run
```
we default to `token0=ETH`, `token1=USDC` and `fee=5`. `arbiter` will return results
```console
Uniswap Pool Result: Uniswap Pool Result: 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640
```
which is the 5BP pools for the pair ETH/USDC on Uniswap. The program runs and streams transactions (swaps) that update the pool's price like so:
```
------------New Swap------------
From pool 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640
Sender: 0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45, Recipient: 0x1019bf2d607cc646a94a194f7a79e0b385065cff
amount_0 -5235133099
amount_1 4335000000000000000
liquidity 23260193077241608585
tick 205351
price "1.208239460504000000000000000000000000000e+3"
```



