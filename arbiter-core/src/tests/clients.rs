use super::*;

#[test]
fn simulation_signer() -> Result<()> {
    let (_manager, client) = startup_user_controlled()?;
    assert_eq!(
        client.address(),
        Address::from_str("0x2efdc9eecfee3a776209fcb8e9a83a6b221d74f5").unwrap()
    );
    Ok(())
}

#[test]
fn multiple_signer_addresses() {
    let params = EnvironmentParameters {
        label: TEST_ENV_LABEL.to_string(),
        block_settings: BlockSettings::UserControlled,
        gas_settings: GasSettings::UserControlled,
    };
    let environment = &mut Environment::new(params);
    environment.run();
    let client_1 = RevmMiddleware::new(environment, Some("0".to_string())).unwrap();
    let client_2 = RevmMiddleware::new(environment, Some("1".to_string())).unwrap();
    assert_ne!(client_1.address(), client_2.address());
}

#[test]
fn signer_collision() {
    let params = EnvironmentParameters {
        label: TEST_ENV_LABEL.to_string(),
        block_settings: BlockSettings::UserControlled,
        gas_settings: GasSettings::UserControlled,
    };
    let environment = &mut Environment::new(params);
    environment.run();
    RevmMiddleware::new(environment, Some("0".to_string())).unwrap();
    assert!(RevmMiddleware::new(environment, Some("0".to_string())).is_err());
}
