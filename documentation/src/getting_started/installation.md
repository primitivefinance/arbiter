# Installation

## Install using Cargo

Once Rust is installed, you can install Arbiter from the package registry using Cargo. To do this, just run:
```bash
cargo install arbiter
```

You can now run `arbiter --version` to verify your installation.

## Building From Source
Install Git, if you haven't already. There are many online guides on how to install Git on different devices, including one from [Github](https://github.com/git-guides/install-git).

Once you're done with the above, you can install Arbiter by cloning the repository. The local crate can then be used to install the Arbiter binary on your machine.

```bash
git clone https://github.com/primitivefinance/arbiter.git
cargo install --path ./arbiter
```

You can now run `arbiter --help` to verify your installation and view the help menu.
