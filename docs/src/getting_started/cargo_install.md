# CLI Installation with Cargo

## Prerequisites
The best way to build Arbiter today is from source. 

Install Git, if you haven't already. There are many online guides on how to install Git on different devices, including one from [Github](https://github.com/git-guides/install-git).

Before installing Arbiter, ensure that you have Rust installed. You can install and verify your Rust installation from the [official website](https://www.rust-lang.org/tools/install).

The Arbiter CLI works alongside [Foundry](https://github.com/foundry-rs/foundry) and aims to provide a similar CLI interface of setting up and interacting with Arbiter projects. Install Foundry from the [official website](https://getfoundry.sh/).

## Installing the Arbiter CLI
Once you're done with the above, you can install Arbiter by cloning the repository. The local crate can then be used to install the Arbiter binary on your machine.

```bash
git clone https://github.com/primitivefinance/arbiter.git
cargo install --path ./arbiter
```

Once this is complete, you can run `arbiter --help` to verify your installation and view the help menu.

## Interacting with the Arbiter CLI
Arbiter provides a Foundry-like CLI experience. You can initialize new projects, generate bindings and execute simulations using the CLI.

To create a new Arbiter project:
```bash
arbiter init your-new-project
cd your-new-project
```

This initializes a new Arbiter project with a template. You can generate the bindings again by running:

```bash
arbiter bind
```

The template is executable at this point and you can run it by running:

```bash
cargo run
```

You can load or write your own smart contracts in the templates `contracts/` directory and begin writing your own simulations. Arbiter treats Rust smart-contract bindings as first-class citizens. The contract bindings are generated via Foundry's forge command. arbiter bind wraps forge with some convenience features that will generate all your bindings to `src/bindings` as a rust module. Foundry power-users are welcome to use forge directly. You can also manage project dependencies using git submodules via `forge install`. The [Foundry book](https://book.getfoundry.sh/reference/forge/forge-install) provides further details on managing project dependencies and other features.