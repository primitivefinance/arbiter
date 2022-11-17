# Arbitrage Module (In Progress)

> A stand-alone Rust program to detect arbitrage events between UniswapV3 and Balancer pools.

## Motivation:

Arbitrage takes advantage of price discrepencies between two (or more) different exchanges. Right now, searchers are able to detect differences in reported prices from DEXs and execute pure profit trades which we call *atomic arbitrage*. The environment for arbitrage is competitive, but for less popular token pairs, there is room for new searchers to bring prices into equilibrium amongst exchanges. 

We want to develop a Rust program that is capable of executing atomic arbitrage transactions for a given token pair between two different DEXs. This will be provided to the community in order to lower the barrier of entry into searching and thus level the playing field a bit more in the arbitrage scene.

## Features (in development):

#### TODOs: 
- [ ] Takes user input of token addresses and finds the corresponding PoolIDs for UniswapV3 and Balancer.
- [ ] Monitors the pool prices continuously and announces when an arbitrage trade with profit exceeding the no-arbitrage bounds + gas cost is found.
- [ ] Executes atomic transactions between pools to capture arbitrage.

## Set up (in development):

#### TODOs:

- [ ] Create a release that takes in CLI user input and streams data to the user in the CLI.
- [ ] Integrate with RPC endpoints to allow for execution of atomic swaps to occur on the network.

## Authors:
Created by [ColinPR](https://github.com/ColinPR) and  [0xJepsen](https://github.com/0xJepsen)
