use std::error::Error;

use clap::Parser;
use itertools_num::linspace;
use serde::{Deserialize, Serialize};
use visualize::{design::*, file_handler::*, plot::*};

use crate::{simulations::OutputStorage, Configurable};

#[derive(Parser, Debug)]
#[clap(about = "Visualize simulation results.")]
pub(crate) struct VisualizeArguments {
    /// Path to the configuration file.
    #[arg(long, default_value = "./configurations/simulate_example.toml", num_args = 0..=1)]
    pub(crate) configuration_path: String,

    /// Subcommands for `simulate`
    #[clap(subcommand)]
    pub(crate) subcommand: VisualizeSubcommand,
}

/// Subcommands for `Visualize`
#[derive(Parser, Serialize, Deserialize, Debug)]
#[clap(about = "Visualizes results of simulations.")]
pub(crate) enum VisualizeSubcommand {
    #[clap(about = "Runs Portfolio simulation.")]
    PricePaths,
    #[clap(about = "Runs UniswapV2 simulation.")]
    LPReturns,
}

pub fn plot_price_data(configuration_file_path: &str) -> Result<(), Box<dyn Error>> {
    // TODO: this file path is for the configuration file which will give information about how the files are named in the output directory.
    let OutputStorage {
        output_path,
        output_file_names,
    } = OutputStorage::configure(configuration_file_path)?;
    let volatility = 0.08;
    let mut curves = vec![];
    for label in 0..10 {
        let output_file = format!(
            "{}/{}_{}_{}.csv",
            output_path, output_file_names, volatility, label
        );
        let liquid_exchange_price_data =
            read_column_from_csv(output_file.as_str(), "liquid_exchange_prices")?;
        let uniswap_price_data = read_column_from_csv(output_file.as_str(), "uniswap_prices")?;
        // println!("{:?}", liquid_exchange_price_data.len());
        let trade_number = linspace(
            0.0,
            liquid_exchange_price_data.len() as f64,
            liquid_exchange_price_data.len(),
        )
        .collect::<Vec<f64>>();

        let liquid_exchange_price_curve = Curve {
            x_coordinates: trade_number.clone(),
            y_coordinates: liquid_exchange_price_data.clone(),
            design: CurveDesign {
                color: Color::Green,
                color_slot: label,
                style: Style::Lines(LineEmphasis::Heavy),
            },
            name: None,
        };

        let uniswap_price_curve = Curve {
            x_coordinates: trade_number,
            y_coordinates: uniswap_price_data,
            design: CurveDesign {
                color: Color::Purple,
                color_slot: label,
                style: Style::Lines(LineEmphasis::Light),
            },
            name: None,
        };

        curves.push(liquid_exchange_price_curve);
        curves.push(uniswap_price_curve);
    }

    let title = "\\text{Price Data}".to_string();
    let axes = Axes {
        x_label: "\\text{Trade Number}".to_string(),
        y_label: "\\text{Price}".to_string(),
        bounds: (
            vec![0.0, curves[0].x_coordinates.len() as f64],
            vec![0.8, 1.6],
        ),
    };
    let display = Display {
        transparent: true,
        mode: DisplayMode::Dark,
        show: true,
    };

    transparent_plot(Some(curves), None, axes, title, display);

    Ok(())
}
