use std::error::Error;
use x11rb::protocol::xproto::ExposeEvent;
use zym_session::common::SessionImpl;

pub fn handle_expose(
    _session: &dyn SessionImpl,
    _event: &ExposeEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
