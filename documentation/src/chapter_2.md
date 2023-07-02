# Chapter 2: On-Chain Crate

The `onchain` crate plays a significant role within our project, primarily focusing on interacting with live Ethereum RPCs. This crate comprises three modules: the `monitor`, the `executor`, and the `objectives`.

## Monitor Module

The `monitor` module's main purpose is to monitor Ethereum logs for specific types of information. Users have the flexibility to filter based on contract address, event name, and block range. Its primary function is data collection and real-time analysis, playing a critical role in the real-time decision-making process.

## Executor Module

The `executor` module deals with the creation and execution of transactions. It is designed to support both Flashbots relayer bundles and standard transactions. The executor module interacts with the blockchain based on the information obtained from the monitor module, thus performing actions in response to real-time data.

## Objectives Module

The `objectives` module defines the objective functions for convex optimization. This module is currently under development with the goal of implementing optimal routing algorithms in Rust.

## Future Directions

We envision that real-time risk monitoring will become crucial for decentralized applications. The `onchain` crate provides a solid foundation to develop this functionality, enabling swift response to real-time data. The crate aims to provide a shared framework for sophisticated on-chain actors, thus accelerating innovation in the space.

## Crate Features

In relation to the `onchain` crate's role in Ethereum RPC interaction, its prominent features are:

* **Real-Time Data Monitoring:** The crate facilitates the real-time analysis of Ethereum logs, with filters based on contract address, event name, and block range.

* **Flexible Transaction Execution:** The crate supports both Flashbots relayer bundles and standard transactions, thus offering versatility in transaction execution.

* **Contract Deployment:** The crate provides a comprehensive contract deployment system, where all necessary contracts for any simulation can be automatically deployed.
