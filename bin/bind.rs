use std::process::Command;

pub(crate) fn bind_forge() -> std::io::Result<()> {
    let output = Command::new("forge")
        .arg("bind")
        .arg("--revert-strings")
        .arg("debug")
        .arg("-b")
        .arg("arbiter/src/bindings/")
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
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Command failed"))
    }
}
