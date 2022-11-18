use clap::{load_yaml, App};

pub fn get_cli() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let token_0 = matches.value_of("TOKEN0").unwrap();
    println!("Value for token_0: {}", token_0);
}