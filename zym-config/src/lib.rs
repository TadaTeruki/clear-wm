use dotenv::dotenv;
use std::env;
use std::error::Error;

pub struct Config {
    logfile: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let logfile_: Option<String>;
        {
            let key = "WM_CONFIG_LOGFILE";
            logfile_ = match env::var(key) {
                Ok(v) => Some(v),
                Err(err) => {
                    println!("{}: {}", err, key);
                    None
                }
            };
        }

        Ok(Self { logfile: logfile_ })
    }

    pub fn get_logfile(&self) -> Option<String> {
        self.logfile.clone()
    }
}
