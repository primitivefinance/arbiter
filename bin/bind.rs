#![warn(missing_docs)]
use std::{env, fs, io, path::Path, process::Command};

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
    bindings_for_submodules(lib_dir)?;

    Ok(())
}

fn bindings_for_submodules(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        // Iterate through entries in the directory
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // If the entry is a directory, run command inside it
            if path.is_dir() && path.file_name().unwrap_or_default() != "forge-std" {
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
    Ok(())
}
