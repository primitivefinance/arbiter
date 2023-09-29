#![warn(missing_docs)]
use std::{
    env, fs,
    fs::{write, File},
    io,
    io::{BufRead, BufReader},
    path::Path,
    process::Command,
};

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
    let mut contracts = collect_contract_list(Path::new("contracts"))?;
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

    // Define path to `lib` directory
    let lib_dir = Path::new("lib");
    let mut list2 = bindings_for_submodules(lib_dir)?;
    contracts.append(&mut list2);
    println!("Contract List: {:?}", contracts);
    remove_unneeded_contracts(contracts)?;

    Ok(())
}

fn bindings_for_submodules(dir: &Path) -> io::Result<Vec<String>> {
    let mut contracts_to_generate = Vec::new(); // to keep track of contracts we're generating bindings for

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
                let output_path = format!("../../src/{}_bindings/", submodule_name);

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
    Ok(contracts_to_generate)
}

fn collect_contract_list(dir: &Path) -> io::Result<Vec<String>> {
    let mut contract_list = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let filename = path.file_stem().unwrap().to_str().unwrap();

                if !filename.starts_with('I') {
                    contract_list.push(filename.to_string().to_lowercase());
                }
            }
        }
    }

    Ok(contract_list)
}

fn remove_unneeded_contracts(needed_contracts: Vec<String>) -> io::Result<()> {
    let binding_dir = Path::new("src/bindings");

    if binding_dir.is_dir() {
        for entry in fs::read_dir(binding_dir)? {
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
    update_mod_file(needed_contracts)?;

    Ok(())
}

fn update_mod_file(contracts_to_keep: Vec<String>) -> io::Result<()> {
    let mod_path = "src/bindings/mod.rs";

    // Open the file and read its contents
    let file = File::open(mod_path)?;
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
    write(mod_path, lines.join("\n"))?;

    Ok(())
}
