#![warn(missing_docs)]
use std::{
    env, fs,
    fs::{write, File},
    io,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::Command,
};

pub(crate) mod digest;
mod test;
use self::digest::ArbiterConfig;
use config::{Config, ConfigError};
use foundry_config::Config as FoundryConfig;
use inflector::Inflector;
use proc_macro2::{Ident, Span};
use serde::Deserialize;

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
    let foundry_config = FoundryConfig::load();
    let arbiter_config = ArbiterConfig::new().unwrap();
    let project_bidnings_output_path = arbiter_config.bindings_path.join("bindings");
    let output = Command::new("forge")
        .arg("bind")
        .arg("--revert-strings")
        .arg("debug")
        .arg("-b")
        .arg(project_bidnings_output_path.clone())
        .arg("--module")
        .arg("--overwrite")
        .output()?;
    let project_contracts = collect_contract_list(&foundry_config.src, &arbiter_config)?;
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("Command output: {}", output_str);
    } else {
        let err_str = String::from_utf8_lossy(&output.stderr);
        println!("Command failed, error: {}, is forge installed?", err_str);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command failed",
        ));
    }
    remove_unneeded_contracts(&project_bidnings_output_path, project_contracts)?;

    if arbiter_config.submodules {
        for lib_dir in &foundry_config.libs {
            println!("Generating bindings for lib: {:?}", lib_dir);
            let (output_path, sub_module_contracts) =
                bindings_for_submodules(lib_dir, &arbiter_config)?;
            remove_unneeded_contracts(&output_path, sub_module_contracts)?;
        }
    }
    Ok(())
}

fn bindings_for_submodules(
    libdir: &Path,
    config: &ArbiterConfig,
) -> io::Result<(PathBuf, Vec<String>)> {
    let mut contracts_to_generate = Vec::new(); // to keep track of contracts we're generating bindings for
    let mut last_output_path = config.bindings_path.clone();

    if libdir.is_dir() {
        // Iterate through entries in the directory
        for entry in fs::read_dir(libdir)? {
            let entry = entry?;
            let path = entry.path();

            // If the entry is a directory, run command inside it
            if path.is_dir() && path.file_name().unwrap_or_default() != "forge-std" {
                contracts_to_generate = collect_contract_list(&path, config)?;

                let submodule_name = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('-', "_");

                let output_path = config
                    .bindings_path
                    .clone() // Get the bindings path from config
                    .canonicalize()? // Convert output to absolute path
                    .join(format!("{}_bindings", submodule_name));

                env::set_current_dir(&path)?;

                println!(
                    "output path: for submodule {:?} is {:?}",
                    submodule_name, &output_path
                );

                let output = Command::new("forge")
                    .arg("bind")
                    .arg("--revert-strings")
                    .arg("debug")
                    .arg("-b")
                    .arg(output_path.clone())
                    .arg("--module")
                    .arg("--overwrite")
                    .output()?;

                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    println!("Command output: {}", output_str);
                } else {
                    let err_str = String::from_utf8_lossy(&output.stderr);
                    println!("Command failed, error: {}", err_str);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Command failed",
                    ));
                }
                last_output_path = output_path;
            }
        }
    }
    Ok((last_output_path, contracts_to_generate))
}

fn collect_contract_list(dir: &Path, settings: &ArbiterConfig) -> io::Result<Vec<String>> {
    let mut contract_list = Vec::new();
    contract_list.push("shared_types".to_string());
    println!("dir value: {:?}", dir.is_dir());
    if dir.is_dir() {
        println!("Collecting contracts from {:?}", dir);
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
                if !settings.ignore_interfaces || !safe_filename.starts_with('i') {
                    contract_list.push(safe_filename);
                }
            }
        }
    }

    Ok(contract_list)
}

fn remove_unneeded_contracts(
    bindings_path: &PathBuf,
    needed_contracts: Vec<String>,
) -> io::Result<()> {
    if bindings_path.is_dir() {
        for entry in fs::read_dir(bindings_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let filename = path.file_stem().unwrap().to_str().unwrap().to_lowercase();

                // Skip if the file is `mod.rs`
                if filename == "mod" || filename == "settings" {
                    continue;
                }

                if !needed_contracts.contains(&filename) {
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
fn safe_ident(name: &str) -> Ident {
    syn::parse_str::<Ident>(name).unwrap_or_else(|_| ident(&format!("{name}_")))
}
/// Creates a new Ident with the given string at [`Span::call_site`].
///
/// # Panics
///
/// If the input string is neither a keyword nor a legal variable name.
fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}
/// converts invalid rust module names to valid ones
fn safe_module_name(name: &str) -> String {
    // handle reserve words used in contracts (eg Enum is a gnosis contract)
    safe_ident(&safe_snake_case(name)).to_string()
}

///  Converts a `&str` to `snake_case` `String` while respecting identifier
/// rules
fn safe_snake_case(ident: &str) -> String {
    safe_identifier_name(ident.to_snake_case())
}

/// respects identifier rules, such as, an identifier must not start with a
/// numeric char
fn safe_identifier_name(name: String) -> String {
    if name.starts_with(char::is_numeric) {
        format!("_{name}")
    } else {
        name
    }
}
