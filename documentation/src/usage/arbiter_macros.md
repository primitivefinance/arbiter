# Arbiter macros
`arbiter_macros` provides a set of macros to help with the use of `arbiter-engine` and `arbiter-core`.
At the moment, we only have one proc macro: `#[derive(Behaviors)]`.
This macro will generate an implementation of a `CreateStateMachine` trait for the `Behaviors` enum and ultimately save you from having to write a lot of boilerplate code.
See the [Configuration](./arbiter_engine/configuration.md) page for more information on how to use this macro.