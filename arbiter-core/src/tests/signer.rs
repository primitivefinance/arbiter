use std::sync::Arc;

use super::*;

#[test]
fn simulation_signer() {
    let environment = &mut Environment::new(TEST_ENV_LABEL, Some(1.0), 1);
    let client = Arc::new(RevmMiddleware::new(
        environment,
        Some(TEST_SIGNER_SEED_AND_LABEL.to_string()),
    ));
    assert_eq!(
        client.default_sender().unwrap(),
        Address::from_str("0x2efdc9eecfee3a776209fcb8e9a83a6b221d74f5").unwrap()
    );
}

#[test]
fn multiple_signer_addresses() {
    let environment = &mut Environment::new(TEST_ENV_LABEL, Some(1.0), 1);
    let client_1 = Arc::new(RevmMiddleware::new(environment, Some("0".to_string())));
    let client_2 = Arc::new(RevmMiddleware::new(environment, Some("1".to_string())));
    assert_ne!(client_1.default_sender(), client_2.default_sender());
}

// TODO: Test to see that we prvent agents with the same name from being added.

#[test]
fn signer_collision() {
    todo!();
}
