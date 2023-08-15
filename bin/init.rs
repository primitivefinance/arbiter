use std::process::Command;
use log::{info, error};
use std::io;

/// Initializes a new Arbiter project from a template.
///
/// This function clones the `arbiter-template` from GitHub into a new directory
/// with the provided project name. After cloning, it performs necessary setup operations.
///
/// # Arguments
///
/// * `name` - The name of the new project. This will also be the name of the directory
/// where the project is initialized.
///
/// # Returns
///
/// Returns an `io::Result<()>` indicating the success or failure of the initialization.
///
/// # Example
///
/// ```
/// use your_crate_name::init_project;
///
/// init_project("my_new_project").expect("Failed to initialize the project");
/// ```

pub fn init_project(name: &str) -> io::Result<()> {

    let status = Command::new("git")
        .arg("clone")
        .arg("https://github.com/primitivefinance/arbiter-template.git")
        .arg(name) // target directory
        .status()?;
        
    if !status.success() {
        error!("Failed to clone the repository.");
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to clone the repository."));
    }
    info!("Your Arbiter project '{}' has been successfully initialized!", name);
    Ok(())
}
