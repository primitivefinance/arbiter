use anyhow::{Ok, Result};
use bytes::Bytes;
use ethers::types::{BlockId, H160 as eH160, H256, U256 as eU256};
use ethers::{
    abi::parse_abi,
    prelude::BaseContract,
    providers::{Http, Provider},
};
use ethers_core::k256::elliptic_curve::rand_core::block;
use ethers_providers::Middleware;
use revm::Bytecode;
use ruint::aliases::{B160, U256 as rU256};
use simulate::testbed::{self, Testbed};
// use revm::db::EthersDB; // BROKEN IMPORT?
use revm::{
    db::{CacheDB, EmptyDB},
    AccountInfo, Database, TransactOut, TransactTo, EVM, KECCAK_EMPTY,
};
use std::{str::FromStr, sync::Arc};
use tokio::runtime::{Handle, Runtime};
use utils::chain_tools::get_provider;

#[tokio::main]
async fn main() -> Result<()> {
    // create a testbed where we can run sims
    let mut testbed = Testbed::new();

    // insert a default user
    let user_address = eH160::from_str("0x0000000000000000000000000000000000000000")?;
    let user_info = revm::AccountInfo::default();
    testbed.db.insert_account_info(user_address, user_info);

    // get contract info TODO: use bindings
    let pool_address = eH160::from_str("0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852")?;
    let slot_use = 8;
    let slot = rU256::from(slot_use);
    let abi = BaseContract::from(
        parse_abi(&[
            "function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)",
        ])?
    );

    // get ethers provider
    let client = get_provider().await;

    // Choose block number to sample data from
    let block_number: u64 = 16434802 as u64;
    let block = Some(BlockId::from(block_number));

    // set up future and call block_on for ethers call
    let index = H256::from(slot.to_be_bytes());
    let f = async {
        let storage = client
            .get_storage_at(pool_address, index, block)
            .await
            .unwrap();
        eU256::from(storage.to_fixed_bytes())
    };
    let value = testbed.block_on(f);

    // encode abi into Bytes
    let encoded = abi.encode("getReserves", ())?;

    // insert our pre-loaded storage slot to the corresponding contract key (address) in the DB
    let f = async {
        let nonce = client.get_transaction_count(pool_address, block);
        let balance = client.get_balance(pool_address, block);
        let code = client.get_code(pool_address, block);
        tokio::join!(nonce, balance, code)
    };
    let (nonce, balance, code) = testbed.block_on(f);
    let pool_acc_info = AccountInfo::new(
        balance.unwrap(),
        nonce
            .unwrap_or_else(|e| panic!("ethers get nonce error: {e:?}"))
            .as_u64(),
        Bytecode::new_raw(
            code.unwrap_or_else(|e| panic!("ethers get code error: {e:?}"))
                .0,
        ),
    );
    testbed.db.insert_account_info(pool_address, pool_acc_info);
    let slot = eU256::from(slot_use);
    testbed
        .db
        .insert_account_storage(pool_address, slot, value)
        .unwrap();

    // perform a transaction
    // let mut tx = revm::TxEnv::default();
    // tx = testbed.perform_transaction(user_address, pool_address, encoded);
    testbed.evm.database(testbed.db);
    testbed.evm.env.tx.caller = user_address;
    testbed.evm.env.tx.transact_to = TransactTo::Call(pool_address);
    testbed.evm.env.tx.data = Bytes::from(hex::decode(hex::encode(&encoded))?);
    testbed.evm.env.tx.value = eU256::from(0);
    let ref_tx = testbed.evm.transact_ref();
    let result = ref_tx.0;

    // unpack output
    let value = match result.out {
        TransactOut::Call(value) => Some(value),
        _ => None,
    };
    let (reserve0, reserve1, ts): (u128, u128, u32) =
        abi.decode_output("getReserves", value.unwrap())?;

    // Print emualted getReserves() call output
    println!("Reserve0: {:#?}", reserve0);
    println!("Reserve1: {:#?}", reserve1);
    println!("Timestamp: {:#?}", ts);

    Ok(())
}
