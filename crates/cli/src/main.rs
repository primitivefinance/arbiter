use anyhow::{Ok, Result};
use bytes::Bytes;
use revm::Bytecode;
use ethers_core::k256::elliptic_curve::rand_core::block;
use ethers_providers::Middleware;
use ruint::aliases::{U256 as rU256, B160};
use ethers::types::{H160 as eH160, H256, BlockId, U256 as eU256};
use ethers::{
    abi::parse_abi,
    prelude::BaseContract,
    providers::{Http, Provider},
};

use revm::{
    db::{CacheDB, EmptyDB},
    AccountInfo, Database, TransactOut, TransactTo, EVM, KECCAK_EMPTY,
};
use std::{str::FromStr, sync::Arc};
use utils::chain_tools::get_provider;
use tokio::runtime::{Handle, Runtime};

#[tokio::main]
async fn main() -> Result<()> {
    /////////////////////////////////////////////////////////////////
    // Initialize a new database to store users and transactions.
    /////////////////////////////////////////////////////////////////
    let mut cache_db = CacheDB::new(EmptyDB {});

    /////////////////////////////////////////////////////////////////
    // Create a user account and fund the account. TODO: fund the account
    /////////////////////////////////////////////////////////////////
    let user_address = eH160::from_str("0x0000000000000000000000000000000000000000")?;
    let user_info = revm::AccountInfo::default();
    cache_db.insert_account_info(user_address, user_info);
    println!("------------CREATE USER ACCOUNT------------");
    println!("Account created: {:#?}", cache_db.accounts);


    /////////////////////////////////////////////////////////////////
    // Add a UniV2Pair contract account to our database and get the
    // getReserves() function
    /////////////////////////////////////////////////////////////////
    let client = get_provider().await;

    // ----------------------------------------------------------- //
    //             Storage slots of UniV2Pair contract             //
    // =========================================================== //
    // storage[5] = factory: address                               //
    // storage[6] = token0: address                                //
    // storage[7] = token1: address                                //
    // storage[8] = (res0, res1, ts): (uint112, uint112, uint32)   //
    // storage[9] = price0CumulativeLast: uint256                  //
    // storage[10] = price1CumulativeLast: uint256                 //
    // storage[11] = kLast: uint256                                //
    // =========================================================== //
    // First call ethersdb.basic() which does...
    // then call ethersdb.storage() calls client.get_storage_at() with curr block number to get stuff
    // ETH/USDT pair on Uniswap V2
    let pool_address = eH160::from_str("0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852")?;
     // Choose slot of storage that you would like to transact with
// let slot = rU256::from(8);
    let slot_use = 8;
    let slot = rU256::from(slot_use);
    // Generate abi for the calldata from the human readable interface
    let abi = BaseContract::from(
        parse_abi(&[
            "function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)",
        ])?
    ); // TODO: this is where we can probably just use contract bindings we already have

     // Choose block number
     let block_number: u64 = 16434802 as u64; //.try_into().unwrap();
     let block = Some(BlockId::from(block_number));
    // essentially COPIED FROM ethersdb for value in example
    // let add = eH160::from(pool_address.0);
    let runtime = Handle::try_current()
            .is_err()
            .then(|| Runtime::new().unwrap());

    // ----- THIS COMES FROM ethersdb.storage()
    let index = H256::from(slot.to_be_bytes());
        let f = async {
            let storage = client
                .get_storage_at(pool_address, index, block)
                .await
                .unwrap();
            // rU256::from_be_bytes(storage.to_fixed_bytes())
            eU256::from(storage.to_fixed_bytes())
        };
    let value = Ok(block_on(&runtime, f)).unwrap();
    // ---- EXIT STUFF FROM THAT FUNCTION
    // println!("Value = {:?}", value);
    
    // encode abi into Bytes
    let encoded = abi.encode("getReserves", ())?;

    // println!("encoded: {:?}", encoded);


    // insert our pre-loaded storage slot to the corresponding contract key (address) in the DB
    let g = async {
        let nonce = client.get_transaction_count(pool_address, block);
        let balance = client.get_balance(pool_address, block);
        let code = client.get_code(pool_address, block);
        tokio::join!(nonce, balance, code)
    };
    let (nonce, balance, code) = block_on(&runtime, g);
        // panic on not getting data?
    let pool_acc_info = AccountInfo::new(
            // rU256::from_limbs(
            //     balance
            //         .unwrap_or_else(|e| panic!("ethers get balance error: {e:?}"))
            //         .0,
            // ),
            balance.unwrap(),
            nonce
                .unwrap_or_else(|e| panic!("ethers get nonce error: {e:?}"))
                .as_u64(),
            Bytecode::new_raw(
                code.unwrap_or_else(|e| panic!("ethers get code error: {e:?}"))
                    .0,
            ),
        );
    cache_db.insert_account_info(pool_address, pool_acc_info);
    let slot = eU256::from(slot_use);
    cache_db
        .insert_account_storage(pool_address, slot, value)
        .unwrap();
    // cache_db.insert_contract();
        // ERROR HERE? The contract doesn't actually get any bytecode?

    // retrieve account from address just to check
    // let pool_acc_info = cache_db.basic(pool_address)?.unwrap();
    let pool_acc_info = cache_db.basic(pool_address).unwrap().unwrap();

    println!("------------CREATE POOL ACCOUNT------------");
    println!("Pool address pulled: {:#?}", pool_acc_info);

    /////////////////////////////////////////////////////////////////
    // Initialise an EVM and perform a transaction on the Uni pool
    /////////////////////////////////////////////////////////////////
    let mut evm = EVM::new();

    // insert pre-built database from above
    evm.database(cache_db);
    println!("---------------PRINT EVM DB---------------");
    println!("{:#?}", evm.db().unwrap());
    // fill in missing bits of env struc
    // change that to whatever caller you want to be
    evm.env.tx.caller = eH160::from_str("0x0000000000000000000000000000000000000000")?;
    // account you want to transact with
    evm.env.tx.transact_to = TransactTo::Call(pool_address);
    // calldata formed via abigen
    evm.env.tx.data = Bytes::from(hex::decode(hex::encode(&encoded))?);
    // transaction value in wei
    evm.env.tx.value = eU256::try_from(0)?;

    // execute transaction without writing to the DB
    let ref_tx = evm.transact_ref();
    // select ExecutionResult struct
    let result = ref_tx.0;


    // unpack output call enum into raw bytes
    let value = match result.out {
        TransactOut::Call(value) => Some(value),
        _ => None,
    };

    println!("Value after call: {:#?}", value.as_ref().unwrap());

    // decode bytes to reserves + ts via ethers-rs's abi decode
    let (reserve0, reserve1, ts): (u128, u128, u32) =
        abi.decode_output("getReserves", value.unwrap())?;
    
    println!("got here");

    // Print emualted getReserves() call output
    println!("Reserve0: {:#?}", reserve0);
    println!("Reserve1: {:#?}", reserve1);
    println!("Timestamp: {:#?}", ts);

    Ok(())
}

fn block_on<F: core::future::Future>(runtime: &Option<Runtime>, f: F) -> F::Output {
    match runtime {
        Some(runtime) => runtime.block_on(f),
        None => futures::executor::block_on(f),
    }
}