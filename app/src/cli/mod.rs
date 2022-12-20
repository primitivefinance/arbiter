use clap::{load_yaml, App};

pub fn get_cli() -> (String, String, String, Option<String>) {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let token_0 = String::from(matches.value_of("token0").unwrap());
    let token_1 = String::from(matches.value_of("token1").unwrap());
    let api_key_reference = matches.value_of("api_key");
    let bp = String::from(matches.value_of("fee").unwrap());
    (token_0, token_1, bp, api_key_reference.map(str::to_string))
}