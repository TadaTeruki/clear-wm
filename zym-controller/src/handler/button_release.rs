use std::error::Error;
use x11rb::protocol::xproto::ButtonReleaseEvent;
use zym_session::common::SessionImpl;

pub fn handle_button_release(
    _session: &dyn SessionImpl,
    _event: &ButtonReleaseEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
