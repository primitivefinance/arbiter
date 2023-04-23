use on_chain::monitor::HistoricalMonitor;
use bindings::uniswap_v3_pool;
use ethers::types::U256;

pub async fn save_backtest_data(_config: &String, start_block: &u64, end_block: &u64, address: &String) {
        let range = *start_block..*end_block;
        let step = 100_u64; // doing this so we don't hit rpc limits
        let contract_address = address;
        let historical_monitor =
            HistoricalMonitor::new(on_chain::monitor::utils::RpcTypes::Mainnet).await;
        let contract_abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.to_owned();
        let mut pricedata: Vec<U256> = Vec::new();
        for block in range.step_by(step as usize) {
            let sqrtpricex96 = historical_monitor
                .historical_monitor(contract_address, contract_abi.clone(), block, block + step)
                .await;
            let sqrtpricex96 = sqrtpricex96.unwrap();
            pricedata.extend(sqrtpricex96)
        }

        historical_monitor
            .save_price_to_csv(&pricedata, "price.csv")
            .unwrap();
}

pub async fn load_backtest_data(_config: &String, file_path: &String) {
    let price_data = simulate::price_simulation::import_price_from_csv(file_path).unwrap();
    let price_ref = &price_data;
    let _ = price_ref;
    // Need to do something with this imported data
}