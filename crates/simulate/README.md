# Simulate Crate

This crate contains agents, price paths, and middleware to interface with revm. There are two primary sub modules in this crate: `agents` and `stochastic`.

- The `agents` module contains the agents that are used in the simulations, this module is where we define the behavior of different agents in the simulations. We have some pre-built agents, but we anticipate that users will want to build their own agents and for this module to grow with different use cases.

- The `stochastic` module contains the price paths and other stochastic processes that are used in the simulations. The existing price paths we support (GBM and OU) are defined here.

- The rest of the files in this crate are concerned with middleware to interface with revm with utils and tools for backtesting with historical data.
