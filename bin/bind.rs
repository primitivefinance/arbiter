#![warn(missing_docs)]
use std::process::Command;

/// Runs the `forge` command-line tool to generate bindings.
///
/// This function attempts to execute the external command `forge` with the
/// provided arguments to generate necessary bindings. The bindings are stored
/// in the `arbiter/src/bindings/` directory, and existing bindings will be overwritten.
/// The function wraps the forge command to generate bindings as a module to a specific destination.
///
/// # Returns
///
/// * `Ok(())` if the `forge` command successfully generates the bindings.
/// * `Err(std::io::Error)` if the command execution fails or if there's an error
///   in generating the bindings. This can also include if the `forge` tool is not installed.

pub(crate) fn forge_bind() -> std::io::Result<()> {
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
        Ok(())
    } else {
        let err_str = String::from_utf8_lossy(&output.stderr);
        println!("Command failed, error: {}, is forge installed?", err_str);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command failed",
        ))
    }
}
