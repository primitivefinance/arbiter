# On-Chain Crate

This crate has three modules concerned with interacting with live Ethereum RPCs: the monitor, the executor, and the objectives.

- The `monitor` module is concerned with monitoring eth logs for various types of information. You can filter by contract address, event name, and block range. The monitor module is used to collect data and perform real-time analysis.
- The `executor` module is concerned with building and executing transactions. The executor supports Flashbots relayer to make bundles as well as standard transactions. The executor module performs actions on the blockchain in response to information from the monitor module.
- The `objectives` module defines the objective functions for convex optimization. This module is still in development. We are building out implementations of the optimal routing algorithms here in Rust.

In the future, we see real-time risk monitoring as very important for decentralized applications. This crate supports the development of that functionality and the ability to respond to the information collected in real-time. We see on-chain sophisticated actors building a lot of the same bespoke infrastructure. We hope to provide a common framework for these actors to accelerate innovation in the space.
