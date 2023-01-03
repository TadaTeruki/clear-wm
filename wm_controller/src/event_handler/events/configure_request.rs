use std::error::Error;
use x11rb::protocol::xproto::ConfigureRequestEvent;

use crate::event_handler::types::WmEventHandler;

impl<'a> WmEventHandler<'a> {
    pub fn handle_configure_request(
        &self,
        event: &ConfigureRequestEvent,
    ) -> Result<(), Box<dyn Error>> {
        // do reflection
        self.event_reflector.reflect_on_configure_request(event)
    }
}
