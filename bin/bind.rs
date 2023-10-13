#![warn(missing_docs)]
use std::{
    env, fs,
    fs::{write, File},
    io,
    io::{BufRead, BufReader},
    path::Path,
    process::Command,
};

use inflector::Inflector;
use proc_macro2::{Ident, Span};

/// Runs the `forge` command-line tool to generate bindings.
///
/// This function attempts to execute the external command `forge` with the
/// provided arguments to generate necessary bindings. The bindings are stored
/// in the `arbiter/src/bindings/` directory, and existing bindings will be
/// overwritten. The function wraps the forge command to generate bindings as a
/// module to a specific destination.
///
/// # Returns
///
/// * `Ok(())` if the `forge` command successfully generates the bindings.
/// * `Err(std::io::Error)` if the command execution fails or if there's an
///   error in generating the bindings. This can also include if the `forge`
///   tool is not installed.

pub(crate) fn forge_bind() -> std::io::Result<()> {
    println!("Generating bindings for project contracts...");
    let output = Command::new("forge")
        .arg("bind")
        .arg("--revert-strings")
        .arg("debug")
        .arg("-b")
        .arg("src/bindings/")
        .arg("--module")
        .arg("--overwrite")
        .output()?;
    let project_contracts = collect_contract_list(Path::new("contracts"))?;
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("Command output: {}", output_str);
        println!("Revert strings are on");
    } else {
        let err_str = String::from_utf8_lossy(&output.stderr);
        println!("Command failed, error: {}, is forge installed?", err_str);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command failed",
        ));
    }

    // TODO: load this path from a config file (foundry.toml)
    let src_binding_dir = Path::new("src/bindings");
    remove_unneeded_contracts(src_binding_dir, project_contracts)?;

    let lib_dir = Path::new("lib");
    let (output_path, sub_module_contracts) = bindings_for_submodules(lib_dir)?;
    println!("submodule contracts: {:?}", sub_module_contracts);
    remove_unneeded_contracts(Path::new(&output_path), sub_module_contracts)?;

    Ok(())
}

fn bindings_for_submodules(dir: &Path) -> io::Result<(String, Vec<String>)> {
    let mut contracts_to_generate = Vec::new(); // to keep track of contracts we're generating bindings for
    let mut output_path = String::new();
    if dir.is_dir() {
        // Iterate through entries in the directory
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // If the entry is a directory, run command inside it
            if path.is_dir() && path.file_name().unwrap_or_default() != "forge-std" {
                contracts_to_generate = collect_contract_list(&path)?;
                println!("Generating bindings for submodule: {:?}...", path);

                env::set_current_dir(&path)?;

                let submodule_name = path.file_name().unwrap().to_str().unwrap(); // Assuming file_name() is not None and is valid UTF-8
                output_path = format!("../../src/{}_bindings/", submodule_name);

                let output = Command::new("forge")
                    .arg("bind")
                    .arg("--revert-strings")
                    .arg("debug")
                    .arg("-b")
                    .arg(&output_path) // Use the dynamically generated path
                    .arg("--module")
                    .arg("--overwrite")
                    .output()?;

                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    println!("Command output: {}", output_str);
                    println!("Revert strings are on");
                } else {
                    let err_str = String::from_utf8_lossy(&output.stderr);
                    println!("Command failed, error: {}", err_str);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Command failed",
                    ));
                }
            }
        }
    }
    Ok((output_path, contracts_to_generate))
}

fn collect_contract_list(dir: &Path) -> io::Result<Vec<String>> {
    let mut contract_list = Vec::new();
    contract_list.push("shared_types".to_string());
    if dir.is_dir() {
        let dir_name = dir.file_name().unwrap().to_str().unwrap(); // Assuming file_name() is not None and is valid UTF-8

        let target_dir = if dir_name == "src" || dir_name == "contracts" {
            dir.to_path_buf()
        } else {
            // Look inside the directory for a directory named "src" or "contracts"
            let potential_src = dir.join("src");
            let potential_contracts = dir.join("contracts");
            if potential_src.is_dir() {
                potential_src
            } else if potential_contracts.is_dir() {
                potential_contracts
            } else {
                return Ok(Vec::new()); // No valid contract directory found
            }
        };

        for entry in fs::read_dir(target_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let filename = path.file_stem().unwrap().to_str().unwrap();
                let valid_name = Ident::new(filename, proc_macro2::Span::call_site());
                let safe_filename = safe_module_name(&valid_name.to_string());
                if !safe_filename.starts_with('i') {
                    contract_list.push(safe_filename);
                }
            }
        }
    }

    Ok(contract_list)
}

fn remove_unneeded_contracts(
    bindings_path: &Path,
    needed_contracts: Vec<String>,
) -> io::Result<()> {
    if bindings_path.is_dir() {
        for entry in fs::read_dir(bindings_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let filename = path.file_stem().unwrap().to_str().unwrap().to_lowercase();

                // Skip if the file is `mod.rs`
                if filename == "mod" {
                    continue;
                }

                if !needed_contracts.contains(&filename) {
                    println!("Removing contract binding: {}", path.display());
                    fs::remove_file(path)?;
                }
            }
        }
    }

    update_mod_file(bindings_path, needed_contracts)?;

    Ok(())
}

fn update_mod_file(bindings_path: &Path, contracts_to_keep: Vec<String>) -> io::Result<()> {
    let mod_path = bindings_path.join("mod.rs");

    // Open the file and read its contents
    let file = File::open(&mod_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| {
            // Keep the line if it's a comment
            if line.trim().starts_with("//") || line.trim().starts_with('#') {
                return true;
            }

            // Check if the line is a module declaration and if it's one of the contracts we
            // want to keep
            if let Some(contract_name) = line
                .trim()
                .strip_prefix("pub mod ")
                .and_then(|s| s.strip_suffix(';'))
            {
                return contracts_to_keep.contains(&contract_name.to_string());
            }

            true
        })
        .collect();

    // Write the new lines back to the mod.rs
    write(&mod_path, lines.join("\n"))?;

    Ok(())
}

/// The following methods were picked out of https://github.com/gakonst/ethers-rs/blob/9d01a9810940d3acd7c78bf2b2f2ca85a74f73eb/ethers-contract/ethers-contract-abigen/src/lib.rs#L393
/// Expands an identifier string into a token and appending `_` if the
/// identifier is for a reserved keyword.
///
/// Parsing keywords like `self` can fail, in this case we add an underscore.
pub(crate) fn safe_ident(name: &str) -> Ident {
    syn::parse_str::<Ident>(name).unwrap_or_else(|_| ident(&format!("{name}_")))
}
/// Creates a new Ident with the given string at [`Span::call_site`].
///
/// # Panics
///
/// If the input string is neither a keyword nor a legal variable name.
pub fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}
/// converts invalid rust module names to valid ones
pub fn safe_module_name(name: &str) -> String {
    // handle reserve words used in contracts (eg Enum is a gnosis contract)
    safe_ident(&safe_snake_case(name)).to_string()
}

// /// Generate the default file name of the module.
// pub fn module_filename() -> String {
//     let mut name = self.module_name();
//     name.push_str(".rs");
//     name
// }

///  Converts a `&str` to `snake_case` `String` while respecting identifier
/// rules
pub(crate) fn safe_snake_case(ident: &str) -> String {
    safe_identifier_name(ident.to_snake_case())
}

/// respects identifier rules, such as, an identifier must not start with a
/// numeric char
pub(crate) fn safe_identifier_name(name: String) -> String {
    if name.starts_with(char::is_numeric) {
        format!("_{name}")
    } else {
        name
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

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
        let dir = tempdir().expect("Failed to create temporary directory");

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
        let contracts = collect_contract_list(dir.path()).expect("Failed to collect contracts");

        // Assert the results
        let expected = vec![
            "shared_types",
            "example_contract",
            "sd5_9x_18_math",
            "g3m",
            "another_test",
        ];
        assert_eq!(contracts, expected);

        // Temp dir will be automatically cleaned up after going out of scope.
    }

    #[test]
    fn test_collect_contract_list_from_src() {
        // Create a temporary directory
        let dir = tempdir().expect("Failed to create temporary directory");

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
        let contracts = collect_contract_list(dir.path()).expect("Failed to collect contracts");

        // Assert the results
        let expected = vec![
            "shared_types",
            "sd5_9x_18_math",
            "example_one",
            "test_two",
            "g3m",
        ];
        assert_eq!(contracts, expected);

        // Temp dir will be automatically cleaned up after going out of scope.
    }
    #[test]
    fn test_update_mod_file() {
        // Create a temporary directory
        let dir = tempdir().expect("Failed to create temporary directory");

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
}
