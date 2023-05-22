# Cli Crate

Simulations are designed here and then driven with `clap` commands in `main.rs`. This crate has three modules: `sim`, `commands`, and `config`.

The sim module is where the simulation is designed. Each sim has three primary components: deploying sim contracts, initialization calls, and the sim config parameters.

- Deploying sim contracts deploys all contracts in the sim scope to a local instance of revm.
- The initialization calls to define the initial state of revm you would like to model. Here you can specify the initial state of the contracts you deployed, what agents you want in your model, what token approvals they have, and the amount of capital they have.
- The sim parameters are determined by the `config` module, which specifies the price path, the number of iterations, and other parameters.

To call the new sim, you need to add it to the `Simulations` enum in `main.rs` and configure the subcommand in `commands.rs` with clap.
