use chrono::Local;
use log::{info, warn, Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::fs::File;
use std::io::Write;
use zym_config::Config;

const WM_LOGGER_STYLE_ERROR: &str = "\x1b[1;31m";
const WM_LOGGER_STYLE_WARN: &str = "\x1b[1;33m";
const WM_LOGGER_STYLE_DEFAULT: &str = "\x1b[1;38m";
const WM_LOGGER_STYLE_END: &str = "\x1b[0m";

static mut WM_LOG_FILE: Option<File> = None;

fn logfile_init(config: &Config) {
    unsafe {
        let filename = match &config.system_config().logfile {
            Some(v) => v,
            None => {
                warn!("the logfile not specified. log will only be written to stdout");
                return;
            }
        };
        WM_LOG_FILE = match File::create(filename) {
            Ok(v) => Some(v),
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        };
        info!(
            "the logfile successfully prepared. now log will be recorded at '{}'.",
            filename
        );
    }
}
pub struct WMLogger;

impl WMLogger {
    pub fn init(&'static self, config: &Config) -> Result<(), SetLoggerError> {
        log::set_logger(self).map(|()| log::set_max_level(LevelFilter::Info))?;
        logfile_init(config);
        Ok(())
    }
}

impl log::Log for WMLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        let level_style = match record.level() {
            Level::Error => WM_LOGGER_STYLE_ERROR,
            Level::Warn => WM_LOGGER_STYLE_WARN,
            _ => WM_LOGGER_STYLE_DEFAULT,
        };

        let current_time = Local::now().format("%Y/%m/%d %H:%M:%S");

        if self.enabled(record.metadata()) {
            println!(
                "[ {} ] {}| {} |{} ( at {} ) - {}",
                current_time,
                level_style,
                record.level(),
                WM_LOGGER_STYLE_END,
                record.target(),
                record.args()
            );
            unsafe {
                if let Some(file) = WM_LOG_FILE.as_mut() {
                    writeln!(
                        file,
                        "[ {} ] | {} | ( at {} ) - {}",
                        current_time,
                        record.level(),
                        record.target(),
                        record.args()
                    )
                    .unwrap_or_else(|err| println!("cannot write log to logfile: {}", err));
                }
            }
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use log::{error, info, warn};
    use zym_config::Config;

    use super::*;

    #[test]
    fn simple_test() {
        static LOGGER: WMLogger = WMLogger;
        let config = Config::load().expect("failed to load configs");
        LOGGER.init(&config).expect("failed to load logger");

        info!("this is information");
        warn!("this is warning");
        error!("nothing occured");
    }
}
