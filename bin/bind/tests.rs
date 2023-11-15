#[allow(unused_imports)]
use super::*;

#[test]
fn test_safe_module_name() {
    assert_eq!(safe_module_name("Valid"), "valid");
    assert_eq!(safe_module_name("Enum"), "enum_");
    assert_eq!(safe_module_name("Mod"), "mod_");
    assert_eq!(safe_module_name("2Two"), "_2_two");
}

#[test]
fn test_collect_contract_list_from_contracts() {
    // Create a temporary directory
    let dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let setting = ArbiterConfig::_new_mock_config();
    // Create nested directories "src" and "contracts"
    let contracts_dir = dir.path().join("contracts");
    fs::create_dir(&contracts_dir).expect("Failed to create contracts directory");

    // Create files in the contracts directory
    fs::write(contracts_dir.join("ExampleContract.sol"), "").expect("Failed to write file");
    fs::write(contracts_dir.join("AnotherTest.sol"), "").expect("Failed to write file");
    fs::write(contracts_dir.join("ITestInterface.sol"), "").expect("Failed to write file"); // This should be ignored
    fs::write(contracts_dir.join("G3M.sol"), "").expect("Failed to write file");
    fs::write(contracts_dir.join("SD59x18Math.sol"), "").expect("Failed to write file");

    // Call the function
    let mut contracts =
        collect_contract_list(dir.path(), &setting).expect("Failed to collect contracts");
    contracts.sort();
    // Assert the results
    let mut expected = vec![
        "shared_types",
        "example_contract",
        "sd5_9x_18_math",
        "g3m",
        "another_test",
        "i_test_interface",
    ];
    expected.sort();
    assert_eq!(contracts, expected);

    // Temp dir will be automatically cleaned up after going out of scope.
}

#[test]
fn test_collect_contract_list_from_src() {
    // Create a temporary directory
    let dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let config = ArbiterConfig::_new_mock_config();

    // Create a nested directory "src"
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir).expect("Failed to create src directory");

    // Create files in the src directory
    fs::write(src_dir.join("ExampleOne.sol"), "").expect("Failed to write file");
    fs::write(src_dir.join("TestTwo.sol"), "").expect("Failed to write file");
    fs::write(src_dir.join("ITestInterface.sol"), "").expect("Failed to write file"); // This should be ignored
    fs::write(src_dir.join("G3M.sol"), "").expect("Failed to write file"); // This should be ignored
    fs::write(src_dir.join("SD59x18Math.sol"), "").expect("Failed to write file"); // This should be ignored

    // Call the function
    let mut contracts =
        collect_contract_list(dir.path(), &config).expect("Failed to collect contracts");
    contracts.sort();
    // Assert the results
    let mut expected = vec![
        "shared_types",
        "sd5_9x_18_math",
        "example_one",
        "test_two",
        "g3m",
        "i_test_interface",
    ];
    expected.sort();
    assert_eq!(contracts, expected);

    // Temp dir will be automatically cleaned up after going out of scope.
}
#[test]
fn test_update_mod_file() {
    // Create a temporary directory
    let dir = tempfile::tempdir().expect("Failed to create temporary directory");

    // Mock a mod.rs file with some content
    let mocked_mod_path = dir.path().join("mod.rs");
    let content = "
    // Some comments
    pub mod example_contract;
    pub mod test_contract;
    ";
    fs::write(&mocked_mod_path, content).expect("Failed to write mock mod.rs file");

    // Call the function
    let contracts_to_keep = vec!["example_contract".to_owned()];
    update_mod_file(mocked_mod_path.parent().unwrap(), contracts_to_keep)
        .expect("Failed to update mod file");

    // Open the mocked mod.rs file and check its content
    let updated_content = fs::read_to_string(&mocked_mod_path).unwrap();
    assert!(updated_content.contains("pub mod example_contract;"));
    assert!(!updated_content.contains("pub mod test_contract;"));

    // Temp dir (and the mock mod.rs file inside it) will be automatically
    // cleaned up after going out of scope.
}

