# Arbiter
**Arbiter** is a framework for stateful Ethereum smart-contract simulation. 
The framework features an [`ethers-rs`](https://github.com/gakonst/ethers-rs) middleware built on top of [revm](https://github.com/bluealloy/revm) which allows the end user to interact with a sandboxed `revm` instance as if it were an Ethereum node. 
This provides a familiar interface for interacting with the Ethereum Virtual Machine (EVM), but with unrivaled speed. 
Furthermore, Arbiter provides containment and management for simulations. For a running list of vulnerabilities found with Arbiter, please see the [Vulnerability Corpus](vulnerability_corpus.md).

## Overview
The Arbiter workspace has three crates:
- `arbiter`: The binary crate that exposes a command line interface for initializing simulations via a templated repository and generating contract bindings needed for the simulation.
- `arbiter-core`: The lib crate that contains the core logic for the Arbiter framework including the `RevmMiddleware` discussed before, the `Environment` which envelopes simulations, and the `Manager` who controls a collection of environments.
- `arbiter-engine`: The lib crate that provides abstractions for building simulations and more.

The purpose of Arbiter is to provide a toolset to construct arbitrary agents (defined in Rust, by smart contracts, or even other foreign function interfaces) and have these agents interact with an Ethereum-like environment of your design. 
All contract bytecode is run directly using a blazing-fast EVM instance `revm` (which is used in live RPC nodes such as [`reth`](https://github.com/paradigmxyz/reth)) so that your contracts are tested in the exact same type of environment that they are deployed in.

## Motivation 
Smart contract engineers need to test their contracts against a wide array of potentially adversarial environments and contract parameters. 
The static stateless testing of contracts can only take you so far. 
To truly test the security of a contract, you need to test it against a wide array of dynamic environments that encompass the externalities of Ethereum mainnet. 
We wanted to do just that with Arbiter. 

Both smart contract and financial engineers come together in Decentralized Finance (DeFi) to build and deploy a wide array of complex decentralized applications as well as financial strategies respectively. 
For the latter, a financial engineer may want to test their strategies against thousands of market conditions, contract settings, shocks, and autonomous or random or even AI agents all while making sure their strategy isn't vulnerable to bytecode-level exploits.

To configure such a rich simulation environment on a test or local network is also possible with Arbiter by a change in choice of middleware. 
The most efficient choice for getting robust, yet quick, simulations would bypass any networking and use a low level language's implementation of the EVM. 
Furthermore, we can gain control over the EVM worldstate by working directly on `revm`.
We would like the user to have a choice in how they want to simulate their contracts and Arbiter provides that choice.

### Sim Driven Development and Strategization 

Test driven development is a popular engineering practice to write tests first, which fail, and implement logic to get the test to eventually pass. 
With simulation driven development, it's possible to build "tests" that can only pass if the *incentives* actually work. For example, a sim driven test might be `is_loan_liquidated`, and a simulation must be made for a liquidator agent to do the liquidation. 
This approach significantly improves the testing of economic systems and other mechanism designs, which is important in the world of networks that are mostly incentive driven.

The same goes with developing strategies that one would like to deploy on a live Ethereum network. 
One can use Arbiter to simulate their strategy with an intended goal and see if it actually works. 
This is especially important in the world of DeFi where strategies are often a mix of on and offchain and are susceptible to exploits.

### Anomaly Detection
Anomaly detection in software design systems refers to identifying unusual patterns or behaviors that deviate from the expected or normal functioning of the software. These anomalies can be due to various reasons, such as bugs, performance issues, security vulnerabilities, or design flaws. Arbiter's agent-based modeling and EVM execution parity make it well suited for anomaly detection of greater systemic risk in the Ethereum ecosystem. 

In the context of software design, anomaly detection can be used to identify design flaws or inconsistencies in the design of the software. For example, if a particular module or component of the software behaves differently than it was intended, it could indicate a design flaw or security vulnerability. 

### Agent Base Modeling 
Agent-based simulations for anomaly detection systems involve creating a model of the system using agents, where each agent represents a component or a module of the system. These agents interact with each other and their environment, mimicking the behavior of the actual system. Agent-based simulations can be a powerful tool for anomaly detection as they can model complex systems and their interactions, making it possible to detect anomalies that other methods might miss. However, they also require a good understanding of the system being modeled and what constitutes normal behavior for that system.

### Modeling the System
The first and most crucial step is to model the system. A well-modeled system accurately reflects the real-world behavior of the software or system under study. This ensures that the simulation provides meaningful and applicable results. We build the `RevmMiddleware` to accurately model how users/agents or externally owned accounts interact with the EVM. This means the `RevmMiddleware` implements the middleware trait from the rust Ethereum ecosystem, exploiting the same API the EOAs would use to talk to a node today. This is why having EVM execution parity is so important.

#### Statistical Methods: 
These methods model the system's normal behavior using statistical models and then use these models to detect deviations. To model things well, people use techniques such as mean, median, and standard deviation, or more complex models like regression models can be used. For example, the Poisson distribution gives the probability of an event happening a certain number of times (k) within a given interval of time or space. So, you can quantify an average number of occurrences of some action (say, to model the behavior of a retail agent or network congestion from certain events). In that case, you can model this well with the Poisson distribution. 

### Defining Normal Behavior: Agent design
Once the system is modeled, the next step is to define what constitutes normal behavior for the system. This could be based on historical data, expert knowledge, or both. This is not a feature of Arbiter yet (The arbiter-engine crate is a WIP but contains some of our initial work on this). This can be incredibly simple (passive behavior) or complex (interactive behavior). But the better they model the system, the better the results. For example, you can model LPs as more passive agents that deposit and withdraw liquidity based on some average occurrences. In contrast, arbitrageurs can be modeled as more interactive agents that react to certain events or `SLAOD's on specific memory locations. As the agents start to resemble real-world actors, the results will be more accurate, and the data will be more beneficial for the system designers.

### Simulating the System
The system is then simulated over some time. During this simulation, the agents interact with each other and their environment, generating data that reflects the system's behavior. You can decide on specific parameters and configurations for the system. Designating the system simulation to be as close to the real-world system as possible is recommended. For example, historically or with price processes, we can model a sequence of prices for arbitrageurs. The speed and performance of the simulation have made it possible for you to get more data by doing the latter. 

### Detecting Anomalies
The data generated by the simulation is then analyzed to detect anomalies. This could be done using various statistical methods, machine learning, or rule-based methods. Anomalies are identified as deviations from the defined normal behavior. 


>Machine Learning: Machine Learning techniques can be used to learn the system's normal behavior and then detect anomalies. 

>Rule-Based Methods: These methods define rules that describe the system's normal behavior. Any behavior that does not conform to these rules is considered an anomaly.

>Time Series Analysis: In systems where data is collected over time, time series analysis can be used to detect anomalies. This involves looking for patterns or trends in the data over time and identifying any deviations from these patterns. This is one way we at Primitive have used Arbiter to ensure the security of EVM-oriented products. 

>>Log Analysis: Many software systems generate logs that record the system's activity. Analyzing these logs can help detect anomalies. This can be done manually or using automated tools. 

>>Evaluating and Refining the Model: The detected anomalies are evaluated to determine if they are true anomalies or false positives. The model is refined based on these evaluations to improve its accuracy in detecting abnormalities.


### Using Insights to Refine the System
Insights gained from the system can be invaluable in refining and improving it. By understanding the anomalies and their causes, we can make necessary adjustments to the system's design or operation. This could involve modifying the system's parameters, updating the agent's behaviors, or even redesigning certain aspects of the system. 

However, it's essential to be cautious about overfitting the data. Overfitting occurs when a model is excessively complex, such as having too many parameters relative to the number of observations. An overfitted model has poor predictive performance, as it overreacts to minor fluctuations in the training data.

## Developer Documentation
To see the documentation for the Arbiter crates, please visit the following:
- [`arbiter`](https://docs.rs/crate/arbiter/)
- [`arbiter-bindings`](https://docs.rs/crate/arbiter-bindings/)
- [`arbiter-core`](https://docs.rs/arbiter-core/)

You will also find each of these on crates.io.