# Chapter 2: On-Chain Crate

The `onchain` crate plays a significant role within our project, primarily focusing on interacting with live Ethereum RPCs. This crate comprises three modules: the `monitor`, the `executor`, and the `objectives`.

## Monitor Module

The `monitor` module's main purpose is to monitor Ethereum logs for specific types of information. Users have the flexibility to filter based on contract address, event name, and block range. Its primary function is data collection and real-time analysis, playing a critical role in the real-time decision-making process.

## Executor Module

The `executor` module deals with the creation and execution of transactions. It is designed to support both Flashbots relayer bundles and standard transactions. The executor module interacts with the blockchain based on the information obtained from the monitor module, thus performing actions in response to real-time data.

## Objectives Module

The `objectives` module defines the objective functions for convex optimization. This module is currently under development with the goal of implementing optimal routing algorithms in Rust.

## Manager Module

The `manager.rs` file contains the `SimulationManager` structure, which is responsible for managing simulations. The `SimulationManager` oversees the simulation environment, activates agents, handles contract deployment, calls contracts, and reads logs. It contains key functions such as `activate_agent`, which adds and activates an agent in the simulation environment, `auto_deploy`, which deploys all contracts necessary for any simulation, and `_deploy_contracts`, a generic contract deployment function.

## Lib Crate Integration

The `lib` crate provides the foundational structures and mechanisms for simulations, integrating closely with the `onchain` crate to enable efficient, live Ethereum interaction.

* The `agent` module in the `lib` crate works alongside the `executor` module in the `onchain` crate to create and execute transactions based on the agent's characteristics and actions.
  
* The `environment` module provides the `SimulationContract` structure used to represent and interact with live contracts on the Ethereum network. It cooperates with the `monitor` module from the `onchain` crate to retrieve real-time data from these contracts.
  
* The `exchange` module in the `lib` crate complements the `Exchange` trait in the `onchain` crate, providing additional functionality for token swaps.
  
* The `historic` module, found in both the `lib` and `onchain` crates, works together to provide an extensive mechanism for working with historical price data.

## Exchange Module

A notable inclusion in our project is the `Exchange` and `Cfmm` traits from the `exchange.rs` file. They describe the functionality of any contract that can be used to swap tokens.

## Historic Module

The `historic.rs` file is essential for generating price paths for a simulation. This allows managers to alter prices for infinitely liquid pools. The public function `import_price_from_csv` allows the importation of price data from a CSV file. This module provides a simple and efficient way to use historical price data in simulations.

## Future Directions

We envision that real-time risk monitoring will become crucial for decentralized applications. The `onchain` crate provides a solid foundation to develop this functionality, enabling swift response to real-time data. The crate aims to provide a shared framework for sophisticated on-chain actors, thus accelerating innovation in the space.

## Crate Features

In relation to the `onchain` crate's role in Ethereum RPC interaction, its prominent features are:

* **Real-Time Data Monitoring:** The crate facilitates the real-time analysis of Ethereum logs, with filters based on contract address, event name, and block range.

* **Flexible Transaction Execution:** The crate supports both Flashbots relayer bundles and standard transactions, thus offering versatility in transaction execution.

* **Optimal Routing Algorithm:** The crate is in the process of developing an optimal routing algorithm. When completed, this feature will enhance efficiency and performance in interacting with the Ethereum network.

* **Interactive Simulations:** The crate allows the management and running of simulations, making it a valuable tool for testing and development. The `SimulationManager` structure oversees the simulation environment and controls various operations, including agent activation, contract deployment, contract calling, and log reading.

* **Contract Deployment:** The crate provides a comprehensive contract deployment system, where all necessary contracts for any simulation can be automatically deployed.

* **Historic Price Data:** The crate supports importing price data from CSV files. This feature is crucial for generating price paths for simulations and for working with historical price data.
