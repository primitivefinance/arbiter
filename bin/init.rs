use std::{io, process::Command};

use super::*;
use crate::ArbiterError;

/// Initializes a new Arbiter project from a template.
///
/// This function does the following:
/// 1. Clones the `arbiter-template` from GitHub into a new directory named
///    after the provided project name.
/// Template link is here https://github.com/primitivefinance/arbiter-template
/// 2. Changes the current directory to the cloned project.
/// 3. Executes the `forge install` command.
///
/// If any of the steps fail, an error is logged and an `io::Error` is returned
/// to the caller.
///
/// # Arguments
///
/// * `name` - The name of the new project. This will also be the name of the
///   directory
/// where the project is initialized.
///
/// # Returns
///
/// Returns an `io::Result<()>` indicating the success or failure of the
/// initialization. Failure can be due to reasons like:
/// - Network issues or repository being unavailable leading to git clone
///   failure.
/// - The `forge install` command failing.

pub(crate) fn init_project(name: &str) -> io::Result<()> {
    let status = Command::new("git")
        .arg("clone")
        .arg("https://github.com/primitivefinance/arbiter-template.git")
        .arg(name)
        .status()?;

    if !status.success() {
        println!("Failed to clone the repository.");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to clone the repository.",
        ));
    }

    env::set_current_dir(name)?;

    let mut cargo_toml_content = fs::read_to_string("Cargo.toml")?;
    cargo_toml_content = cargo_toml_content.replace("arbiter_template", name);
    // Write the modified Cargo.toml back to disk
    fs::write("Cargo.toml", cargo_toml_content)?;

    let install_output = Command::new("forge").arg("install").output()?;

    if install_output.status.success() {
        let output_str = String::from_utf8_lossy(&install_output.stdout);
        println!("Command output: {}", output_str);
    } else {
        let err_str = String::from_utf8_lossy(&install_output.stderr);
        println!("Command failed, error: {}, is forge installed?", err_str);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command failed",
        ));
    }

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
        println!("Note: revert strings are on");
    } else {
        let err_str = String::from_utf8_lossy(&output.stderr);
        println!("Command failed, error: {}, is forge installed?", err_str);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command failed",
        ));
    }

    println!(
        "Your Arbiter project '{}' has been successfully initialized!",
        name
    );
    Ok(())
}
/// Removes the `.git` directory from the current working directory.
///
/// This function executes the `rm` command with the `-rf` flag to remove
/// the `.git` directory. The function is designed to be used in scenarios
/// where you want to uninitialize a Git repository without deleting the
/// working directory.
///
/// # Returns
///
/// - `Ok(())` if the `.git` directory is successfully removed.
/// - `Err(Box<dyn Error>)` if the `rm` command fails to execute or if the
///   `.git` directory could not be removed.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - The `rm` command is not found or cannot be executed.
/// - The `.git` directory does not exist or cannot be removed due to permission
///   issues.
pub fn remove_git() -> Result<(), ArbiterError> {
    Command::new("rm").arg("-rf").arg(".git").status()?;
    Ok(())
}
