///! Utility functions for casting between revm and ethers-rs types.
use ethers::types::Address;
use revm::primitives::B160;

/// Converts logs from the Revm format to the Ethers format.
///
/// This function iterates over a list of logs as they appear in the `revm` and
/// converts each log entry to the corresponding format used by the `ethers-rs`
/// library.
#[inline]
pub fn revm_logs_to_ethers_logs(
    revm_logs: Vec<revm::primitives::Log>,
) -> Vec<ethers::core::types::Log> {
    let mut logs: Vec<ethers::core::types::Log> = vec![];
    for revm_log in revm_logs {
        let topics = revm_log.topics.into_iter().map(recast_b256).collect();
        let log = ethers::core::types::Log {
            address: recast_address(revm_log.address),
            topics,
            data: ethers::core::types::Bytes::from(revm_log.data),
            block_hash: None,
            block_number: None,
            transaction_hash: None,
            transaction_index: None,
            log_index: None,
            transaction_log_index: None,
            log_type: None,
            removed: None,
        };
        logs.push(log);
    }
    logs
}

// Certainly will go away with alloy-types
/// Recast a B160 into an Address type
/// # Arguments
/// * `address` - B160 to recast. (B160)
/// # Returns
/// * `Address` - Recasted Address.
#[inline]
pub fn recast_address(address: B160) -> Address {
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}

/// Recast a B256 into an H256 type
/// # Arguments
/// * `input` - B256 to recast. (B256)
/// # Returns
/// * `H256` - Recasted H256.
#[inline]
pub fn recast_b256(input: revm::primitives::B256) -> ethers::types::H256 {
    let temp: [u8; 32] = input.as_bytes().try_into().unwrap();
    ethers::types::H256::from(temp)
}