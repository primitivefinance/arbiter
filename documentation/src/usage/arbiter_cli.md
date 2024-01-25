# Arbiter CLI
Arbiter provides a Foundry-like CLI experience. You can initialize new projects, generate bindings and execute simulations using the CLI.

To create a new Arbiter project:
```bash
arbiter init your-new-project
cd your-new-project
```
This initializes a new Arbiter project with a template.
You can run `arbiter init <simulation_name> --no-git` to remove the `.git` directory from the template upon initialization.

## Bindings

You can load or write your own smart contracts in the `arbiter-bindings/contracts/` directory and begin writing your own simulations. 
Arbiter treats Rust smart-contract bindings as first-class citizens. 
The contract bindings are generated via Foundry's `forge` command. 
`arbiter bind` wraps `forge` with some convenience features that will generate all your bindings to src/bindings as a rust module. 
[Foundry](https://github.com/foundry-rs/foundry) power-users are welcome to use `forge` directly.
You can generate the bindings again by running:

```bash
arbiter bind
```
Arbiter bind wraps `forge bind` and is configured from your cargo.toml. There are three optional fields you can add to your toml to configure arbiter bind. 
```toml
[arbiter]
bindings_workspace = "simulation" # must be a valid workspace member
submodules = false # change to true if you want the submodule bindings to be generated
ignore_interfaces = false # change to true if you want to ignore interfaces contracts
```

The template is executable at this point and you can run it by running:

```bash
cargo run
```

You can load or write your own smart contracts in the templates `contracts/` directory and begin writing your own simulations. Arbiter treats Rust smart-contract bindings as first-class citizens. The contract bindings are generated via Foundry's forge command. arbiter bind wraps forge with some convenience features that will generate all your bindings to `src/bindings` as a rust module. Foundry power-users are welcome to use forge directly. You can also manage project dependencies using git submodules via `forge install`. The [Foundry book](https://book.getfoundry.sh/reference/forge/forge-install) provides further details on managing project dependencies and other features.


## Forking

To fork a state of an EVM network, you must first create a fork config file.
An example is provided in the `example_fork` directory.
Essentially, you provide your storage location for the data, the network you want the block number you want, and metadata about the contracts you want to fork.

```bash
arbiter fork <fork_config.toml>
```

This will create a fork of the network you specified in the config file and store it in the location you specified.
It can then be loaded into an `arbiter-core` `Environment` by using the `Fork::from_disk()` method.

Forking is done this way to make sure that all emulation done does not require a constant connection to an RPC-endpoint.

**Optional Arguments** 
You can run `arbiter fork <fork_config.toml> --overwrite` to overwrite the fork if it already exists.