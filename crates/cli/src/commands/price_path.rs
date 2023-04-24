#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::error::Error;

use simulate::stochastic::price_process::{Plotting, PriceProcess, PriceProcessType, GBM, OU};

use crate::config::{ConfigGBM, ConfigOU};

/// Generate a [`GBM`] price path with parameters sourced from [`config.toml`] and plot it
/// # Arguments
/// * [`config_path`] - Path to the [`config.toml`] file
pub fn plot_gbm(config_path: &String) -> Result<(), Box<dyn Error>> {
    let config = ConfigGBM::new(config_path)?;
    let gbm_config = GBM::new(config.drift, config.volatility);
    let price_gbm = PriceProcess::new(
        PriceProcessType::GBM(gbm_config),
        config.timestep,
        config.timescale,
        config.num_steps,
        config.initial_price,
        config.seed,
    );
    let (time, price_path) = price_gbm.generate_price_path();
    price_gbm.plot(&time, &price_path);
    Ok(())
}

/// Generate a [`OU`] price path with parameters sourced from [`config.toml`] and plot it
/// # Arguments
/// * [`config_path`] - Path to the [`config.toml`] file
pub fn plot_ou(config_path: &String) -> Result<(), Box<dyn Error>> {
    let config = ConfigOU::new(config_path)?;
    let ou_config = OU::new(
        config.volatility,
        config.ou_mean_reversion_speed,
        config.ou_mean,
    );
    let price_ou = PriceProcess::new(
        PriceProcessType::OU(ou_config),
        config.timestep,
        config.timescale,
        config.num_steps,
        config.initial_price,
        config.seed,
    );
    let (time, price_path) = price_ou.generate_price_path();
    price_ou.plot(&time, &price_path);
    Ok(())
}
