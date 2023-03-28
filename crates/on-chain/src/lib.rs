// pub mod executor; TODO Work on executor next
pub mod monitor;

#[cfg(test)]
mod tests {
    use crate::monitor::{EventMonitor, utils::RpcTypes};
    use eyre::Result;
    use bindings::i_uniswap_v3_pool;

    // Example contract ABI and bytecode
    const CONTRACT_ADDRESS: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";

    #[tokio::test]
    /// Test the event monitor
    async fn test_event_monitor() -> Result<()> {
        
        // Instantiate the EventMonitor
        let event_monitor = EventMonitor::new(RpcTypes::Mainnet).await;


        // Put this is in its own thread 
        event_monitor.monitor_events(CONTRACT_ADDRESS, i_uniswap_v3_pool::IUNISWAPV3POOL_ABI.clone()).await?;
        // Put console reader in a thread
        
        // if it reads an event then sucess
    
        Ok(())
    }
}