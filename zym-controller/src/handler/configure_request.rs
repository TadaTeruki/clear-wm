use std::error::Error;
use x11rb::protocol::xproto::ConfigureRequestEvent;
use zym_session::common::SessionImpl;

pub fn handle_configure_request(
    session: &dyn SessionImpl,
    event: &ConfigureRequestEvent,
) -> Result<(), Box<dyn Error>> {
    session.configure_window(event)?;
    Ok(())
}
