# Arbiter Core
The `arbiter-core` crate is the core of the Arbiter framework. 
It contains the `Environment` struct which acts as an EVM sandbox and the `RevmMiddleware` which gives a convenient interface for interacting with contracts deployed into the `Environment`.
The API provided by `RevmMiddleware` is that of the `Middleware` trait in the `ethers-rs` crate, therefore it looks and feels just like you're interacting with a live network when you work with an Arbiter `Environment`. 
The only notable differences are in the control you have over this `Environment` compared to something like Anvil, a testnet, or a live network.
