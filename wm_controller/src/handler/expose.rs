use std::error::Error;
use x11rb::protocol::xproto::ExposeEvent;

use super::WmHandler;

impl<'a> WmHandler<'a> {
    pub fn handle_expose(&mut self, event: &ExposeEvent) -> Result<(), Box<dyn Error>> {
        self.client_usecase.draw_client(event.window)?;
        Ok(())
    }
}
