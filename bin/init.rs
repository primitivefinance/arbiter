use std::{env, io, process::Command};

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

    println!(
        "Your Arbiter project '{}' has been successfully initialized!",
        name
    );
    Ok(())
}
