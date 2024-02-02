use super::*;

pub(crate) const TEST_ENV_LABEL: &str = "test";
const TEST_CONTRACT_SIZE_LIMIT: usize = 42069;
const TEST_GAS_LIMIT: u64 = 1_333_333_333_337;

#[test]
fn new_with_parameters() {
    let environment = Environment::builder()
        .with_label(TEST_ENV_LABEL)
        .with_contract_size_limit(TEST_CONTRACT_SIZE_LIMIT)
        .with_gas_limit(U256::from(TEST_GAS_LIMIT));
    assert_eq!(environment.parameters.label, Some(TEST_ENV_LABEL.into()));
    assert_eq!(
        environment.parameters.contract_size_limit.unwrap(),
        TEST_CONTRACT_SIZE_LIMIT
    );
    assert_eq!(
        environment.parameters.gas_limit.unwrap(),
        U256::from(TEST_GAS_LIMIT)
    );
}

#[test]
fn conversion() {
    // Test with a value that fits in u64.
    let input = U256::from(10000);
    assert_eq!(convert_uint_to_u64(input).unwrap(), U64::from(10000));

    // Test with a value that is exactly at the limit of u64.
    let input = U256::from(u64::MAX);
    assert_eq!(convert_uint_to_u64(input).unwrap(), U64::from(u64::MAX));

    // Test with a value that exceeds the limit of u64.
    let input = U256::from(u64::MAX) + U256::from(1);
    assert!(convert_uint_to_u64(input).is_err());
}
