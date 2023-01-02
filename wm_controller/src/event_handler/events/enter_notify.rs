use std::error::Error;
use x11rb::protocol::xproto::EnterNotifyEvent;

use crate::event_handler::handler::WmEventHandler;

impl<'a> WmEventHandler<'a> {
    pub fn handle_enter_notify(&self, _event: &EnterNotifyEvent) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
