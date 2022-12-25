use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    system_config: SystemConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemConfig {
    pub logfile: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let args: Vec<_> = env::args().collect();
        if args.len() <= 1 {
            panic!("could not open the zym configuration file.");
        }

        let config_file = args[1].clone();

        let parsed_system_config: SystemConfig;
        {
            let system_config_file = config_file + "/zym-system.json";
            println!("{}", system_config_file);
            let raw_system_config = fs::read_to_string(system_config_file)?;
            parsed_system_config = serde_json::from_str(&raw_system_config)?;
        }

        Ok(Self {
            system_config: parsed_system_config,
        })
    }

    pub fn system_config(&self) -> &SystemConfig {
        &self.system_config
    }
}
