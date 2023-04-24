use std::error::Error;

use bindings::uniswap_v3_pool;
use on_chain::monitor::EventMonitor;

pub async fn chain() -> Result<(), Box<dyn Error>> {
    // Parse the contract address for a UniswapV3 pool.
    let contract_address = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
    let event_monitor = EventMonitor::new(on_chain::monitor::utils::RpcTypes::Mainnet).await;
    let contract_abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.clone();
    let _ = event_monitor
        .monitor_events(contract_address, contract_abi)
        .await;
    Ok(())
}
