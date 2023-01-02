use std::error::Error;
use x11rb::protocol::xproto::EnterNotifyEvent;

use super::WmHandler;

impl<'a> WmHandler<'a> {
    pub fn handle_enter_notify(&self, _event: &EnterNotifyEvent) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
