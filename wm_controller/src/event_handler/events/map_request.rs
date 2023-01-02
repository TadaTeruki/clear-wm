use std::error::Error;
use x11rb::protocol::xproto::MapRequestEvent;

use crate::event_handler::handler::WmEventHandler;

impl<'a> WmEventHandler<'a> {
    pub fn handle_map_request(&mut self, event: &MapRequestEvent) -> Result<(), Box<dyn Error>> {
        self.client_usecase.compose_client(event.window)?;
        self.client_usecase.activate_client(event.window)?;
        Ok(())
    }
}
