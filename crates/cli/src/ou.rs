use simulate::stochastic::price_process::{
    Price,
    PriceProcessType,
    OU,
    Plotting,
};

use crate::config::ConfigOU;

pub fn plot_ou(config_path: &String) {
    let config = ConfigOU::new(config_path).unwrap();
    let price_ou = Price::new(
        PriceProcessType::OU,
        config.timestep,
        config.timescale,
        config.num_steps,
        config.initial_price,
        config.seed,
    );
    let ou_config = OU::new(config.volatility, config.ou_mean_reversion_speed, config.ou_mean);
    let time = ou_config.generate_ou(
        price_ou.timestep,
        price_ou.num_steps,
        price_ou.initial_price,
        price_ou.seed,
    ).0;
    let price_path = ou_config.generate_ou(
        price_ou.timestep,
        price_ou.num_steps,
        price_ou.initial_price,
        price_ou.seed,
    ).1;
    price_ou.plot(&time, &price_path);
}