use std::error::Error;
use x11rb::protocol::xproto::UnmapNotifyEvent;

use super::WmHandler;

impl<'a> WmHandler<'a> {
    pub fn handle_unmap_notify(&self, _event: &UnmapNotifyEvent) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
