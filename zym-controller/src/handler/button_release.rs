use super::WmHandler;
use std::error::Error;
use x11rb::protocol::xproto::ButtonReleaseEvent;

impl<'a> WmHandler<'a> {
    pub fn handle_button_release(&self, _event: &ButtonReleaseEvent) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
