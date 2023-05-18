#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::error::Error;

use bindings::uniswap_v3_pool;
use ethers::types::U256;
use onchain::monitor::HistoricalMonitor;

/// Save historical sqrt_price_x96 data from a Uniswap V3 pool contract to a csv file.
/// # Arguments
/// * `config` - Path to the config file. (&str)
/// * `start_block` - Start block to get data from. (u64)
/// * `end_block` - End block to get data from. (u64)
/// * `address` - Address of the Uniswap V3 pool contract to monitor. (String)
pub async fn save_backtest_data(
    _config: &str,
    start_block: &u64,
    end_block: &u64,
    address: &str,
) -> Result<(), Box<dyn Error>> {
    let range = *start_block..*end_block;
    let step = 100_u64; // doing this so we don't hit rpc limits
    let contract_address = address;
    let historical_monitor =
        HistoricalMonitor::new(onchain::monitor::utils::RpcTypes::Mainnet).await;
    let contract_abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.to_owned();
    let mut pricedata: Vec<U256> = Vec::new();
    for block in range.step_by(step as usize) {
        let sqrtpricex96 = historical_monitor
            .historical_monitor(contract_address, contract_abi.clone(), block, block + step)
            .await;
        let sqrtpricex96 = sqrtpricex96.unwrap();
        pricedata.extend(sqrtpricex96)
    }

    historical_monitor.save_price_to_csv(&pricedata, "price.csv")?;
    Ok(())
}

/// Load historical price data from a csv file as Vec<f64> and do something with it.
/// # Arguments
/// * `config` - Path to the config file. (String)
/// * `file_path` - Path to the csv file. (String)
pub async fn load_backtest_data(_config: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let price_data = simulate::historic::import_price_from_csv(file_path)?;
    let price_ref = &price_data;
    let _ = price_ref;
    // Need to do something with this imported data
    Ok(())
}
