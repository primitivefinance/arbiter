use std::str::FromStr;

use super::*;

const FORK_CONFIG_PATH: &str = "test_config.toml";
const PATH_TO_DISK_STORAGE: &str = "./output/test.json";

#[test]
fn create_forked_db() {
    let fork_config = ForkConfig::new(FORK_CONFIG_PATH).unwrap();
    let forked_db = ForkedDB::new(&fork_config).unwrap();
    assert!(!forked_db.0.accounts.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn fork_into_arbiter() {
    let fork_config = ForkConfig::new(FORK_CONFIG_PATH).unwrap();
    let forked_db = ForkedDB::new(&fork_config).unwrap();

    // Get the environment going
    let mut environment = environment::builder::EnvironmentBuilder::new()
        .db(forked_db)
        .build();
    environment.run();

    // Create a client
    let client = Arc::new(RevmMiddleware::new(&environment, Some("name")).unwrap());

    // Deal with the weth contract
    let weth_data = fork_config.contracts.get("weth").unwrap();
    let weth = crate::tests::weth::WETH::new(weth_data.address, client.clone());

    let address_to_check =
        Address::from_str(&weth_data.mappings.get("balanceOf").unwrap()[0]).unwrap();
    let balance = weth.balance_of(address_to_check).call().await.unwrap();
    assert_eq!(balance, U256::from(34890707020710109111_u128));
}

#[test]
fn write_out() {
    let fork_config = ForkConfig::new(FORK_CONFIG_PATH).unwrap();
    fork_config.to_disk().unwrap();
}

#[test]
fn read_in() {
    // First write out so we know the file exists.
    let fork_config = ForkConfig::new(FORK_CONFIG_PATH).unwrap();
    fork_config.to_disk().unwrap();

    let forked_db = ForkedDB::from_disk(PATH_TO_DISK_STORAGE).unwrap();
    println!("{:#?}", forked_db);
}
