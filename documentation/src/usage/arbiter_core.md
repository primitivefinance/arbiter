# Arbiter Core
The `arbiter-core` crate is the core of the Arbiter framework. 
It contains the `Environment` struct which acts as an EVM sandbox and the `RevmMiddleware` which gives a convenient interface for interacting with contracts deployed into the `Environment`.
The API provided by `RevmMiddleware` is that of the `Middleware` trait in the `ethers-rs` crate, therefore it looks and feels just like you're interacting with a live network when you work with an Arbiter `Environment`. 
The only notable differences are in the control you have over this `Environment` compared to something like Anvil, a testnet, or a live network.

## Environment
The `Environment` owns a `revm` instance for processing EVM bytecode.
To make the `Environment` performent and flexible, it runs on its own system thread and receives all communication via `Instruction`s sent to it via a `Sender<Instruction>`.

### Instructions
`Instruction`s have been added to over time, but at the moment we allow for the following:
- `Instruction::AddAccount`: Add an account to the `Environment`'s world state. This is usually called by the `RevmMiddleware` when a new client is created.
- `Instruction::BlockUpdate`: Update the `Environment`'s block number and block timestamp. This can be handled by an external agent in a simulation, if desired.
- `Instruction::Cheatcode`: Execute one of the `Cheatcodes` on the `Environment`'s world state. 
The `Cheatcodes` include:
    - `Cheatcodes::Deal`: Used to set the raw ETH balance of a user. Useful when you need to pay gas fees in a transaction.
    - `Cheatcodes::Load`: Gets the value of a storage slot of an account. 
    - `Cheatcodes::Store`: Sets the value of a storage slot of an account.
    - `Cheatcodes::Access`: Gets the account at an address.
- `Instruction::Query`: Allows for querying the `Environment`'s world state and current configuration. Anything in the `EnvironmentData` enum is accessible via this instruction.
    - `EnvironmentData::BlockNumber`: Gets the current block number of the `Environment`.
    - `EnvironmentData::BlockTimestamp`: Gets the current block timestamp of the `Environment`.
    - `EnvironmentData::GasPrice`: Gets the current gas price of the `Environment`.
    - `EnvironmentData::Balance`: Gets the current ETH balance of an account.
    - `EnvironmentData::TransactionCount`: Gets the current nonce of an account.
- `Instruction::Stop`: Stops the `Environment`'s thread and echos out to any listeners to shut down their event streams. This can be used when handling errors or reverts, or just when you're done with the `Environment`.
- `Instruction::Transaction`: Executes a transaction on the `Environment`'s world state. This is usually called by the `RevmMiddleware` when a client sends a ETH-call or state-changing transaction.

The `RevmMiddleware` provides methods for sending the above instructions to an associated `Environment` so that you do not have to interact with the `Environment` directly!

### Events
The `Environment` also emits Ethereum events and errors/reverts to clients who are set to listen to them. 
To do so, we use a `tokio::sync::broadcast` channel and the `RevmMiddleware` manages subscriptions to these events.
As for errors or reverts, we are working on making the flow of handling these more graceful so that your own program or agents can decide how to handle them.

## RevmMiddleware
The `RevmMiddleware` is the main interface for interacting with an `Environment`.