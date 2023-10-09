use std::str::FromStr;

use arbiter_core::{
    bindings::weth,
    environment::{builder::EnvironmentBuilder, fork::Fork},
    middleware::RevmMiddleware,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::*;

const FORK_CONFIG_PATH: &str = "example_fork/weth_config.toml";
const PATH_TO_DISK_STORAGE: &str = "example_fork/test.json";

#[test]
fn create_forked_db() {
    let fork_config = ForkConfig::new(FORK_CONFIG_PATH).unwrap();
    let fork = fork_config.into_fork().unwrap();
    assert!(!fork.db.accounts.is_empty());
}

#[test]
fn write_out() {
    let fork_config = ForkConfig::new(FORK_CONFIG_PATH);
    assert!(fork_config.is_ok());
    let fork_config = fork_config.unwrap();

    // Use par_iter to parallelize the loop
    (0..100).into_par_iter().for_each(|_| {
        let disk_op = fork_config.clone().write_to_disk(&true);
        assert!(disk_op.is_ok());
    });

    let remove_op = fs::remove_file(PATH_TO_DISK_STORAGE);
    assert!(remove_op.is_ok());
}

#[test]
fn read_in() {
    // First write out so we know the file exists.
    let fork_config = ForkConfig::new(FORK_CONFIG_PATH);
    assert!(fork_config.is_ok());
    let fork_config = fork_config.unwrap();
    let disk_op = fork_config.write_to_disk(&true);
    assert!(disk_op.is_ok());
    assert!(Path::new(PATH_TO_DISK_STORAGE).exists());

    // Use par_iter to parallelize the loop
    (0..100).into_par_iter().for_each(|_| {
        let forked_db = Fork::from_disk(PATH_TO_DISK_STORAGE);
        assert!(forked_db.is_ok());
    });
    let remove_op = fs::remove_file(PATH_TO_DISK_STORAGE);
    assert!(remove_op.is_ok());
}

#[tokio::test]
async fn fork_into_arbiter() {
    // for some reason i couldn't get this to work by generating a new fork and
    // writing it to disk but if it exists on disk it works fine
    let fork = Fork::from_disk("example_fork/fork_into_test.json").unwrap();

    // Get the environment going
    let environment = EnvironmentBuilder::new().db(fork.db).build();

    // Create a client
    let client = RevmMiddleware::new(&environment, Some("name")).unwrap();

    // Deal with the weth contract
    let weth_meta = fork.contracts_meta.get("weth").unwrap();
    let weth = weth::WETH::new(weth_meta.address, client.clone());

    let address_to_check_balance =
        Address::from_str(&weth_meta.mappings.get("balanceOf").unwrap()[0]).unwrap();

    println!("checking address: {}", address_to_check_balance);
    let balance = weth
        .balance_of(address_to_check_balance)
        .call()
        .await
        .unwrap();
    assert_eq!(balance, U256::from(34890707020710109111_u128));
}
