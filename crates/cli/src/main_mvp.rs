use std::str::FromStr;

use bytes::Bytes;
use ethers::types::{H160 as eH160, U256 as eU256};
use eyre::Result;
use revm::{AccountInfo, Bytecode, TransactTo};
use simulate::testbed::Testbed;

use utils::chain_tools::get_provider;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    // Do a transaction using revm
    // CLIENT STUFF WE NEED TO GET RID OF
    let client = get_provider().await;

    // create a testbed where we can run sims
    let mut testbed = Testbed::new();

    // insert a default user
    let user_addr = eH160::from_str("0x0000000000000000000000000000000000000001")?;
    let user_acc_info = AccountInfo::new(
        eU256::from(1293874298374982736983074_u128),
        0,
        Bytecode::new(),
    );
    testbed.create_user(user_addr);
    testbed
        .evm
        .db()
        .unwrap()
        .insert_account_info(user_addr, user_acc_info);

    // Get contracts bytes
    let contract_bytes = bindings::hello_world::HELLOWORLD_BYTECODE.to_owned();
    println!("{:#?}", contract_bytes);

    let contract_bytes = contract_bytes.to_vec();
    let contract_bytes = Bytes::from(contract_bytes);

    let contract_bytecode = Bytecode::new_raw(contract_bytes);
    println!("{:#?}", contract_bytecode);

    // Get initialization code from bindings (in future will try to do this manually without a client)
    let contract_deployer = bindings::hello_world::HelloWorld::deploy(client, ()).unwrap();
    let initialization_bytes = contract_deployer.deployer.tx.data().unwrap();
    let initialization_bytes = Bytes::from(hex::decode(hex::encode(initialization_bytes))?);

    // execute initialization code from user
    testbed.evm.env.tx.caller = user_addr;
    testbed.evm.env.tx.transact_to = TransactTo::create();
    testbed.evm.env.tx.data = initialization_bytes;
    testbed.evm.env.tx.value = eU256::zero();
    let result = testbed.evm.transact().0;

    println!("Printing value from TransactOut: {:#?}", result);
    Ok(())
}
