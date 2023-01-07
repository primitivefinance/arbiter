use std::{fs};
use toml;
use serde::{Serialize, Deserialize};


// ! this is the struct that will be used to read the config.toml table
#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    network: Option<ConfigTomlNetwork>,
    see: Option<ConfigTomlSee>
}
// ! this is the struct that will be used to read the network config.toml values
#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlNetwork {
    rpc_url: Option<String>
}
// ! this is the struct that will be used to read the see config.toml values
#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlSee {
    token0 : Option<String>,
    token1 : Option<String>,
    bp : Option<String>,
}
// ! this is the struct that will be used to represent the config.toml values and is returned 
#[derive(Debug)]
pub struct Config {
    pub rpc_url: String,
    pub token0: String,
    pub token1: String,
    pub bp: String
}

impl Config {
    // ! when makin a new instance of the config object we need to read the config.toml file and then return the values
    pub fn new() -> Self {
        let config_filepaths: [&str; 2] = [
            "./crates/cli/src/config.toml",
            "./crates/cli/src/Config.toml",
        ];

        let mut content: String = "".to_owned();
        // ! Read from list of filepaths until we find a file that exists
        for filepath in config_filepaths {
            let result = fs::read_to_string(filepath);

            println!("{result:?}");

            match result {
                Ok(c) => { content = c },
                Err(_) => break,
            }
        }

        // ! initialize the outer struct with values from the config.toml file
        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml object from config file");
            ConfigToml{
                network: None,
                see: None
            }
        });

        let rpc = match config_toml.network {
            Some(network) => network.rpc_url.unwrap_or_else(|| {
                    println!("Missing Field rpc_url in network config");
                    "unknown".to_owned()
                }),
            None => "unknown".to_owned(),
            
        };
        // ! check the token0, token1, and bp fields in the see config.toml section
        let (token0, token1,bp) = match config_toml.see {
            Some(see) => {
                let token0 = see.token0.unwrap_or_else(|| {
                    println!("Missing Field token0 in see config");
                    "unknown".to_owned()
                });
                let token1 = see.token1.unwrap_or_else(|| {
                    println!("Missing Field token1 in see config");
                    "unknown".to_owned()
                });
                let bp = see.bp.unwrap_or_else(||{
                    println!("Missing Field bp in see config");
                    "unknown".to_owned()
                });
                (token0, token1, bp)
            }
            None => ("unknown".to_owned(), "unknown".to_owned(), "unknown".to_owned())
        };

        // return the config object
        Config { 
            rpc_url: rpc,
            token0: token0,
            token1: token1,
            bp: bp,
        }
    }
}