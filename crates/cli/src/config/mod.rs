use std::{fs};
use ethers::abi::token;
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
                let rpc_url = network.rpc_url.unwrap_or_else(|| {
                    println!("Missing Field rpc_url in network config");
                    "unknown".to_owned()
                });
                rpc_url
            },
            None => "unknown".to_owned(),
            
        };
        let (token0, token1, bp) = match config_toml.see {
            Some(see) => {
                let token0 = see.token0.unwrap_or_else(|| {
                    println!("Missing Field token0 in see config");
                    "unknown".to_owned()
                });
                let token1 = see.token1.unwrap_or_else(|| {
                    println!("Missing Field token1 in see config");
                    "unknown".to_owned()
                });
                let bp = see.bp.unwrap_or_else(|| {
                    println!("Missing Field bp in see config");
                    "unknown".to_owned()
                });
                (token0, token1, bp)
            },
            None => ("unknown".to_owned(), "unknown".to_owned(), "unknown".to_owned())
            
        };

        println!("{}", rpc);

        Config { 
            rpc_url: rpc,
            token0: token0,
            token1: token1,
            bp: bp,
        }
    }
}