pub mod error;
pub mod session;

use crate::session::WMSession;
use log::info;
use std::error::Error;
use x11rb::xcb_ffi::XCBConnection;

pub fn start_session() -> Result<(), Box<dyn Error>> {
    let (connection, screen_num) = XCBConnection::connect(None)?;
    info!("established connection to X server");

    let session = WMSession::new(&connection, screen_num);
    session.set_as_wm()?;

    loop {
        session.handle_event()?;
    }
}
