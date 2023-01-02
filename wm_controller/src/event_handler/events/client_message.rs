use std::error::Error;
use x11rb::protocol::xproto::ClientMessageEvent;

use crate::event_handler::handler::WmEventHandler;

impl<'a> WmEventHandler<'a> {
    pub fn handle_client_message(
        &mut self,
        _event: &ClientMessageEvent,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
