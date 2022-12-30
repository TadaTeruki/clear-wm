use std::error::Error;
use x11rb::protocol::xproto::ButtonPressEvent;
use zym_session::common::ClientSessionImpl;

pub fn handle_button_press(
    _session: &dyn ClientSessionImpl,
    _event: &ButtonPressEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
