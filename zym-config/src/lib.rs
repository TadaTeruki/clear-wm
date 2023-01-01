use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    build: BuildConfig,
    wm: WmConfig,
}

#[derive(Deserialize, Debug)]
pub struct BuildConfig {
    pub logfile: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct WmControlConfig {
    pub drag_corner_width: i32,
}

#[derive(Deserialize, Debug)]
pub struct WmFrameConfig {
    pub titlebar_height: i32,
    pub border_width: i32,
    pub border_radius: i32,
}

#[derive(Deserialize, Debug)]
pub struct WmClientConfig {
    pub frame: WmFrameConfig,
    pub control: WmControlConfig,
}

#[derive(Deserialize, Debug)]
pub struct WmConfig {
    pub client: WmClientConfig,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let args: Vec<_> = env::args().collect();
        if args.len() <= 1 {
            panic!("could not open the zym configuration file.");
        }

        let config_file = &args[1].clone();

        let parsed_build_config: BuildConfig;
        {
            let file = config_file.to_string() + "/zym-build.json";
            println!("{}", file);
            let raw = fs::read_to_string(file)?;
            parsed_build_config = serde_json::from_str(&raw)?;
        }

        let parsed_wm_config: WmConfig;
        {
            let file = config_file.to_string() + "/zym-wm.json";
            println!("{}", file);
            let raw = fs::read_to_string(file)?;
            parsed_wm_config = serde_json::from_str(&raw)?;
        }

        Ok(Self {
            build: parsed_build_config,
            wm: parsed_wm_config,
        })
    }

    pub fn build_config(&self) -> &BuildConfig {
        &self.build
    }

    pub fn wm_config(&self) -> &WmConfig {
        &self.wm
    }
}
