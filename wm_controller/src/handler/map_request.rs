use std::error::Error;
use x11rb::protocol::xproto::MapRequestEvent;

use super::WmHandler;

impl<'a> WmHandler<'a> {
    pub fn handle_map_request(&mut self, event: &MapRequestEvent) -> Result<(), Box<dyn Error>> {
        self.client_usecase.compose_client(event.window)?;
        self.client_usecase.activate_client(event.window)?;
        Ok(())
    }
}
