# Introduction

Arbiter is a free an open source EVM analysis tool for complex smart contract systems. At it's core it is an in memory EVM sanbox with a set of analysis tools built on top of it. It is designed to be modular, extensible and performant. Arbiter is capable of delivering an array of stachastic analysis toold with evm execution parity. 

## Motivation
Smart contract engineers need to test their contracts against a wide array of potentially adversarial environments and contract parameters. The static stateless testing of contracts can only take you so far. To truly test the security of a contract, you need to test it against a wide array of dynamic environments that encompass the externalities of Ethereum mainnet. We wanted to do just that with Arbiter.

Both smart contract and financial engineers come together in Decentralized Finance (DeFi) to build and deploy a wide array of complex decentralized applications as well as fincancial strategies respectively. For the latter, a financial engineer may want to test their strategies against thousands of market conditions, contract settings, shocks, and autonomous or random or even AI agents all while making sure their strategy isn't vulnerable to bytecode-level exploits.

To configure such a rich simulation environment on a test or local network is also possible with Arbiter by a change in choice of middleware. The most efficient choice for getting robust, yet quick, simulations would bypass any networking and use a low level language's implementation of the EVM. Furthermore, we can gain control over the EVM worldstate by working directly on revm. We would like the user to have a choice in how they want to simulate their contracts and Arbiter provides that choice.