use std::error::Error;
use x11rb::protocol::xproto::{ConfigureRequestEvent, ConfigureWindowAux, ConnectionExt};

use super::{reflector::WmEventReflector, traits::EventReflectorImpl};

impl<'a> EventReflectorImpl<'a> for WmEventReflector<'a> {
    fn reflect_on_configure_request(
        &self,
        event: &ConfigureRequestEvent,
    ) -> Result<(), Box<dyn Error>> {
        let aux = ConfigureWindowAux::new()
            .x(event.x as i32)
            .y(event.y as i32)
            .width(event.width as u32)
            .height(event.height as u32)
            .sibling(None)
            .stack_mode(None);
        self.connection.configure_window(event.window, &aux)?;
        Ok(())
    }
}
