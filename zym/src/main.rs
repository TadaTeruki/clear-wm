use log::{error, info, warn};
use zym_config::Config;
use zym_logger::WMLogger;
use zym_session::start_session;

static LOGGER: WMLogger = WMLogger;

fn main() {
    let config = Config::load().expect("failed to load configs");
    LOGGER.init(&config).expect("failed to load a logger");

    match start_session() {
        Ok(_) => info!("session was successfully finished"),
        Err(err) => {
            error!("{}", err);
            warn!("session was interrupted");
        }
    };
}
