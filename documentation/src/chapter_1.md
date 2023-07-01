# Chapter 1: Simulate Crate (Interface over revm)

```revm``` is an EVM written is Rust that focuses on **speed** and **simplicity**.

Features, **interfacing** - ```no_std``` so that it can be used as a wasm lib to integrate with Javascript and cpp bindings if necessary.

## Features of the Simulate Crate

The primary features of the `simulate` crate are as follows:

* **Interfacing:** Designed with an open API, the `simulate` crate provides an accesible gateway to `revm` functionality, thus allowing developers to easily incorpate EVM capabilities into their applications.

* **Compatibility:** With a `no_std` design, `simulate` can be compiled to a WebAssembly (wasm) library, facilitating seamless integration with Javascript. Furthermore, this design also provides C++ bindings if required, thereby enhancing its cross-language adaptability.

* **Manager Module:** The `manager.rs` file contains the `SimulationManager` structure, which is responsible for managing simulations. The `SimulationManager` oversees the simulation environment, activates agents, handles contract deployment, calls contracts, and reads logs.

* **Lib Crate Integration:** The `lib` crate provides foundational structures and mechanisms for simulations, integrating closely with the `onchain` crate for efficient, live Ethereum interaction.

* **Exchange Module:** The `Exchange` and `Cfmm` traits from the `exchange.rs` file describe the functionality of any contract that can be used to swap tokens.

* **Historic Module:** The `historic.rs` file is key for generating price paths for a simulation, allowing managers to alter prices for infinitely liquid pools. This module also allows for the importation of price data from a CSV file.

The `simulate` crate constitutes a key component of our Rust ecosystem, primarily dealing with agent-based simulations, price paths, and middleware that interfaces with the `revm`.

## Primary Submodules

There are two primary submodules in the `simulate` crate:

* **Agents:** This module plays host to the various agents that form an integral part of our simulations. Agent behavior is distinctly defined here and even though a number of pre-built agents are included, we expect users to leverage this module to create and incorporate their own agents. This paves the way for continued growth and diverse use cases.

* **Stochastic:** As a pivotal submodule of the `simulate` crate, `stochastic` is where price paths and other stochastic processes crucial for simulations are defined. Currently, we support Gaussian Geometric Brownian Motion (GBM) and Ornstein-Uhlenbeck (OU) price paths.

Aside from these two submodules, the `simulate` crate also includes a variety of middleware utilities and tools designed to interface with `revm`. This includes tools for backtesting using historical data.

## Crate Features

Given the `simulate` crate's role in interfacing with `revm`, its key features can be outlined as follows:

* **Customizable Agent Behavior:** The `simulate` crate allows for the definition of various agent behaviors, enabling unique simulation scenarios.

* **Diverse Stochastic Processes:** The crate supports various stochastic processes, such as GBM and OU price paths, providing a wide array of possibilities for simulations.

* **Robust Middleware Tools:** With a suite of middleware utilities, `simulate` facilitates interaction with `revm` and enables backtesting with historical data.

* **Extensibility:** The crate is designed with user customization in mind, enabling developers to define their own agents and processes to expand upon the existing pre-built options.

* **Interactive Simulations:** The crate allows the management and running of simulations, making it a valuable tool for testing and development. The `SimulationManager` structure oversees the simulation environment and controls various operations, including agent activation, contract deployment, contract calling, and log reading.

* **Historic Price Data:** The crate supports importing price data from CSV files. This feature is crucial for generating price paths for simulations and for working with historical price data.

* **Optimal Routing Algorithm:** The crate is in the process of developing an optimal routing algorithm. When completed, this feature will enhance efficiency and performance in interacting with the Ethereum network.