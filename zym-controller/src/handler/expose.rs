use std::error::Error;
use x11rb::protocol::xproto::ExposeEvent;

use super::WmHandler;

impl<'a> WmHandler<'a> {
    pub fn handle_expose(&self, _event: &ExposeEvent) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
