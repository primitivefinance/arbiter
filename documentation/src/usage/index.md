# Usage
Usage of the Arbiter framework is split by each crate.

## Arbiter CLI
The Arbiter CLI is a minimal interface for managing your Arbiter projects.
It is built on top of Foundry and aims to provide a similar CLI interface of setting up and interacting with Arbiter projects.

## Arbiter Core
The `arbiter-core` crate is the core of the Arbiter framework.
It contains the `Environment` struct which acts as an EVM sandbox and the `RevmMiddleware` which gives a convenient interface for interacting with contracts deployed into the `Environment`.
(TODO) Direct usage of `arbiter-core` will be minimized as much as possible as it is intended for developers to mostly pull from the `arbiter-engine` crate in the future.

## Arbiter Engine (TODO)
The `arbiter-engine` crate is the main interface for running simulations.
It is built on top of `arbiter-core` and provides a more ergonomic interface for designing agents and running them in simulations.