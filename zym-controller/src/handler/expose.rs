use std::error::Error;
use x11rb::protocol::xproto::ExposeEvent;
use zym_session::common::ClientSessionImpl;

pub fn handle_expose(
    _session: &dyn ClientSessionImpl,
    _event: &ExposeEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
