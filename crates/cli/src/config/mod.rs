use std::{fs};
use toml;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    network: Option<ConfigTomlNetwork>,
    see: Option<ConfigTomlSee>
}
#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlNetwork {
    rpc_url: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlSee {
    token0 : Option<String>,
    token1 : Option<String>,
    bp : Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub rpc_url: String,
    pub token0: String,
    pub token1: String,
    pub bp: String
}
impl Config {
    pub fn new() -> Self {

        let config_filepaths: [&str; 2] = [
        "./config.toml",
        "./Config.toml",
        ];

        let mut content: String = "".to_owned();

        for filepath in config_filepaths {
            let result = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        // outer struct init
        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml object from config file");
            ConfigToml{
                network: None,
                see: None
            }
        });


        let rpc = match config_toml.network {
            Some(network) => {
                let network_rpc = network.rpc_url.unwrap_or_else(|| {
                    println!("Missing Field rpc_url in network config");
                    "unknown".to_owned()
                });
                network_rpc
            },
            None => "unknown".to_owned(),
            
        };

        println!("{}", rpc);

        Config { 
            rpc_url: "test".to_owned(),
            token0: "test".to_owned(),
            token1: "test".to_owned(),
            bp: "test".to_owned(),
        }
    }
}