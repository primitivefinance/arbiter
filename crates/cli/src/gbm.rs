use simulate::stochastic::price::{
    Price,
    PriceProcessType,
    GBM
};

use crate::config::ConfigGBM;

pub fn plot_gbm(config_path: &String) {
    let config = ConfigGBM::new(config_path).unwrap();
    let price_gbm = Price::new(
        PriceProcessType::GBM,
        config.timestep,
        config.timescale,
        config.num_steps,
        config.initial_price,
        config.seed,
    );
    let gbm_config = GBM::new(config.drift, config.volatility);
    let time = gbm_config::generateGBM(
        price_gbm.timestep,
        price_gbm.num_steps,
        price_gbm.initial_price,
        price_gbm.seed,
    ).0;
    let price_path = gbm_config::generateGBM(
        price_gbm.timestep,
        price_gbm.num_steps,
        price_gbm.initial_price,
        price_gbm.seed,
    ).1;
    price_gbm.plot(&time, &price_path);
}

