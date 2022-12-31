use std::error::Error;
use x11rb::protocol::xproto::ButtonPressEvent;

use super::WmHandler;

impl<'a> WmHandler<'a> {
    pub fn handle_button_press(&self, _event: &ButtonPressEvent) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
