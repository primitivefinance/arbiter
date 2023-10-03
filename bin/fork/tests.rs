use arbiter_core::{environment, middleware::RevmMiddleware};

use super::*;

const FORK_CONFIG_PATH: &str = "example_fork/weth_config.toml";
const PATH_TO_DISK_STORAGE: &str = "./output/test.json";

#[test]
fn create_forked_db() {
    let fork_config = ForkConfig::new(FORK_CONFIG_PATH).unwrap();
    let forked_db = fork_config.to_db().unwrap();
    assert!(!forked_db.0.accounts.is_empty());
}

#[test]
fn write_out() {
    let fork_config = ForkConfig::new(FORK_CONFIG_PATH).unwrap();
    fork_config.to_disk(&true).unwrap();
}

#[test]
fn read_in() {
    // First write out so we know the file exists.
    let fork_config = ForkConfig::new(FORK_CONFIG_PATH).unwrap();
    fork_config.to_disk(&true).unwrap();

    let forked_db = ForkedDB::from_disk(PATH_TO_DISK_STORAGE).unwrap();
    println!("{:#?}", forked_db);
}
