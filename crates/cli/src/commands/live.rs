#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::error::Error;

use bindings::uniswap_v3_pool;
use on_chain::monitor::EventMonitor;

/// Monitor events from a smart contract.
pub async fn live() -> Result<(), Box<dyn Error>> {
    let contract_address = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
    let event_monitor = EventMonitor::new(on_chain::monitor::utils::RpcTypes::Mainnet).await;
    let contract_abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.clone();
    event_monitor
        .monitor_events(contract_address, contract_abi)
        .await?;
    Ok(())
}
