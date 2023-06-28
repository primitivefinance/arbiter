use std::fs;
use std::io::Write;
use std::path::Path;
use chrono::{DateTime, Utc, Datelike};


pub(crate) fn create_simulation(simulation_name: &str) -> std::io::Result<()> {
    let now: DateTime<Utc> = Utc::now();

    let content = r#"fn main() { 
        println!("Hello, world!");
}"#;

    let toml = format!(r#"[package]
name = "{} Simulation"
version = "0.1.0"
edition = "{}"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]"#, simulation_name, now.year());
    // Create a directory
    fs::create_dir_all("arbiter")?;

    // Create a subdirectory
    let bindings_path = Path::new("arbiter").join("bindings");
    fs::create_dir_all(bindings_path)?;

    let simulations_path = Path::new("arbiter").join("simulations");
    fs::create_dir_all(simulations_path)?;

    let src_path = Path::new("arbiter").join("src");
    fs::create_dir_all(&src_path)?;

    // Create a file in the subdirectory
    let file_path = Path::new("arbiter").join("Cargo.toml");
    let mut file = fs::File::create(file_path)?;
    write!(file, "{}", toml)?;

    let file_path = src_path.join("main.rs");
    let mut file = fs::File::create(file_path)?;
    write!(file, "{}", content)?;

    Ok(())
}

#[test]
fn main() {
    create_simulation("portfolio").unwrap();
}


