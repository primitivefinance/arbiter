# Arbitrage Module (In Progress)

> A stand-alone Rust program to detect and execute arbitrage events between UniswapV3 and ...

## Motivation:

Arbitrage takes advantage of price discrepencies between two (or more) different exchanges. Right now, searchers are able to detect differences in reported prices from DEXs and execute pure profit trades which we call *atomic arbitrage*. The environment for arbitrage is competitive, but for less popular token pairs, there is room for new searchers to bring prices into equilibrium amongst exchanges. 

We want to develop a Rust program that is capable of detecting and executing on abritrage opprtunities between different exchanges. This will be provided to the community in order to lower the barrier of entry into searching and thus level the playing field a bit more in the arbitrage scene.

## Features (in development):
 
#### TODOs: 
- [ ] Library of top 10 popular tokens which includes symbol, address, and decimals. 
- [ ] Takes user input of token addresses and finds the corresponding PoolIDs for UniswapV3.
- [ ] Monitors the UniswapV3 pool prices continuously.
- [ ] Add additional exchanges and stream their prices continuously.
- [ ] Announces when an arbitrage trade with profit exceeding the no-arbitrage bounds + gas cost is found between two exchanges.
- [ ] Executes atomic transactions between pools to capture arbitrage.

## Set up (in development):

`arbitrage-module` takes in three command line arguments. To see the available arguments, run:
```console
$ cargo run -- -h
```
This will display the `help` menu
```console
arbitrage-module 0.0.1

USAGE:
    app [FLAGS] [OPTIONS]

FLAGS:
        --api_key    Provide an Etherscan API key if function calls require it (e.g., providing a token that is not in
                     the database).
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --token0 <token0>    Specifies the first token for a token pair. [default: ETH]
        --token1 <token1>    Specifies the second token for a token pair. [default: USDC]
```

In the above, `token0` and `token1` will be the token pair used to find the corresponding UniswapV3 pools. Upon running
```console
$ cargo run
```
we default to `token0=ETH` and `token1=USDC`. `arbitrage-module` will return results
```console
Uniswap Pool Result: [
    0xe0554a476a092703abdb3ef35c80e0d76d32939f,
    0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640,
    0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8,
    0x7bea39867e4169dbe237d55c8242a8f2fcdcc387,
]
```
which are the 1BP, 5BP, 30BP, and 100BP pools for the pair ETH/USDC. The program runs and streams transactions (swaps) that update the pool's price.

#### TODOs:
- [ ] Provide `make` installation instructions and a brief "how to get started". 
- [ ] Create a release that takes in CLI user input and streams data to the user in the CLI.
- [ ] Allow for users to input an Etherscan API key to pull token info from the chain.
- [ ] Integrate with user selected RPC endpoint.

