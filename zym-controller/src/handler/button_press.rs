use std::error::Error;
use x11rb::protocol::xproto::ButtonPressEvent;
use zym_session::common::SessionImpl;

pub fn handle_button_press(
    _session: &dyn SessionImpl,
    _event: &ButtonPressEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
