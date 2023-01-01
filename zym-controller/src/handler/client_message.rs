use super::WmHandler;

use std::error::Error;
use x11rb::protocol::xproto::ClientMessageEvent;

impl<'a> WmHandler<'a> {
    pub fn handle_client_message(
        &mut self,
        _event: &ClientMessageEvent,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
