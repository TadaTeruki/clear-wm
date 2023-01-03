use std::error::Error;
use x11rb::protocol::xproto::ExposeEvent;

use crate::event_handler::types::WmEventHandler;

impl<'a> WmEventHandler<'a> {
    pub fn handle_expose(&mut self, event: &ExposeEvent) -> Result<(), Box<dyn Error>> {
        self.client_usecase.draw_client(event.window)?;
        Ok(())
    }
}
