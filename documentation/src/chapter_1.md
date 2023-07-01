# Chapter 1: Interface over revm (simulate crate)

```revm``` is an EVM written is Rust that focuses on **speed** and **simplicity**.

Features, **interfacing** - ```no_std``` so that it can be used as a wasm lib to integrate with Javascript and cpp bindings if necessary.

## Features of the simulate crate

The primary features of the `simulate` crate are as follows:

* **Interfacing:** Designed with an open API, the `simulate` crate provides an accesible gateway to `revm` functionality, thus allowing developers to easily incorpate EVM capabilities into their applications.

* **Compatibility:** With a `no_std` design, `simulate` can be compiled to a WebAssembly (wasm) library, facilitating seamless integration with Javascript. Furthermore, this design also provides C++ bindings if required, thereby enhancing its cross-language adaptability.


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


