// pub mod executor; TODO Work on executor next
pub mod monitor;

#[cfg(test)]
mod tests {
    use crate::monitor::EventMonitor;

    use super::*;
    use ethers::prelude::{builders::Deployer, Signer, Wallet};
    use ethers::providers::{Provider, Http};
    use eyre::Result;
    use bindings::i_uniswap_v3_pool;

    // Example contract ABI and bytecode
    const CONTRACT_ADDRESS: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";

    #[tokio::test]
    async fn test_event_monitor() -> Result<()> {
        // Start a Ganache testnet
        let event_monitor = EventMonitor::default().await;

        // Create a provider with the Ganache UR

        // Instantiate the EventMonitor
        event_monitor.monitor_events(CONTRACT_ADDRESS, i_uniswap_v3_pool::IUNISWAPV3POOL_ABI.clone()).await?;
        
        Ok(())
    }
}