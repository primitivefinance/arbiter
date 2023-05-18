pub mod configuration;
pub mod portfolio;
pub mod uniswap;

#[derive(Parser, Debug)]
#[clap(about = "Runs simulations")]
struct SimArgs {
    /// Path to config.toml containing simulation parameterization (optional)
    #[arg(short, long, default_value = "./crates/cli/src/config.toml", num_args = 0..=1)]
    config: Option<String>,

    /// Subcommands for `Sim`
    #[clap(subcommand)]
    subcommand: SimSubcommands,
}

/// Subcommands for `Sim`
#[derive(Subcommand, Debug, PartialEq)]
enum SimSubcommands {
    #[clap(about = "Runs portfolio simulation")]
    Portfolio,
    #[clap(about = "Runs Uniswap V3 simulation")]
    Uniswap,
}