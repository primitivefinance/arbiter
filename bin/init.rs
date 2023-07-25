use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};
use toml::value::{Table, Value};

const MAIN: &str = r#"
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    todo!()
}
"#;

pub(crate) fn create_simulation(simulation_name: &str) -> std::io::Result<()> {
    // move the src dir to contracts
    fs::rename("src", "contracts")?;

    // Update path
    update_import_paths("test/Counter.t.sol")?;

    // change foundry.toml to point to contracts
    rewrite_foundry_toml()?;

    // make the dirs
    fs::create_dir_all("src/bindings")?;
    fs::create_dir_all("src/simulations")?;
    let sim_path = format!("src/simulations/{}", simulation_name);
    fs::create_dir_all(sim_path)?;

    // Add bidings to gitignore
    add_to_gitignore("src/bindings")?;

    // write the cargo toml
    write_cargo_toml(simulation_name)?;

    // Write to mod.rs
    let file_path = format!("src/simulations/{}/mod.rs", simulation_name);
    let mut file = fs::File::create(file_path)?;
    file.write_all(format!("pub mod {};", simulation_name).as_bytes())?;

    // Write to main.rs
    let mut file = fs::File::create("src/main.rs")?;
    file.write_all(MAIN.as_bytes())?;

    Ok(())
}

fn write_cargo_toml(name: &str) -> std::io::Result<()> {
    let mut cargo_toml = Table::new();

    // [package]
    let mut package = Table::new();
    package.insert("name".to_string(), Value::String(name.to_string()));
    package.insert("version".to_string(), Value::String("0.1.0".to_string()));
    package.insert("edition".to_string(), Value::String("2021".to_string()));
    cargo_toml.insert("package".to_string(), Value::Table(package));

    // [[bin]]
    let mut bin = Table::new();
    bin.insert("name".to_string(), Value::String(name.to_string()));
    bin.insert(
        "path".to_string(),
        Value::String("arbiter/src/main.rs".to_string()),
    );
    cargo_toml.insert("bin".to_string(), Value::Array(vec![Value::Table(bin)]));

    // [dependencies]
    let mut simulate = Table::new();
    simulate.insert(
        "git".to_string(),
        Value::String("https://github.com/primitivefinance/arbiter".to_string()),
    );
    simulate.insert("package".to_string(), Value::String("simulate".to_string()));
    let mut dependencies = Table::new();
    dependencies.insert("simulate".to_string(), Value::Table(simulate));
    cargo_toml.insert("dependencies".to_string(), Value::Table(dependencies));

    // Write to a file
    let toml = toml::to_string(&cargo_toml).expect("Failed to serialize the TOML");
    let mut file = File::create("Cargo.toml").expect("Could not create file");
    file.write_all(toml.as_bytes())
        .expect("Could not write to file");
    Ok(())
}

fn _write_arbiter_toml() -> std::io::Result<()> {
    todo!("write arbiter toml");
}

fn rewrite_foundry_toml() -> std::io::Result<()> {
    // Load existing file
    let mut file = File::open("foundry.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse toml and modify it
    let mut config: Value = contents.parse().expect("Failed to parse TOML");
    config["profile"]["default"]["src"] = Value::String("contracts".to_string());

    // Write modified config back to file
    let modified_config = toml::to_string(&config).expect("Failed to serialize TOML");
    let mut file = File::create("foundry.toml")?;
    file.write_all(modified_config.as_bytes())?;

    Ok(())
}

fn update_import_paths(file_path: &str) -> std::io::Result<()> {
    // Read the file to a String.
    let content = fs::read_to_string(file_path)?;

    // Replace "src" with "contracts".
    let updated_content = content.replace("src", "contracts");

    // Write the updated content back to the file.
    fs::write(file_path, updated_content)?;

    Ok(())
}

fn add_to_gitignore(path: &str) -> std::io::Result<()> {
    // Open the file in append mode (or create it if it doesn't exist yet)
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(".gitignore")?;

    // Add a newline and the path to the end of the file
    writeln!(file, "{}", path)?;

    Ok(())
}
