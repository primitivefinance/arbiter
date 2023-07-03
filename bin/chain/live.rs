#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::error::Error;

use simulate::bindings::uniswap_v3_pool;
use monitor::EventMonitor;

use super::monitor;

// TODO: This should use the configuration file and structs in `onchain/mod.rs`
/// Monitor live events from a Uniswap V3 pool contract.
pub async fn run() -> Result<(), Box<dyn Error>> {
    let contract_address = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
    let event_monitor = EventMonitor::new(monitor::utils::RpcTypes::Mainnet).await;
    let contract_abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.clone();
    event_monitor
        .monitor_events(contract_address, contract_abi)
        .await?;
    Ok(())
}
