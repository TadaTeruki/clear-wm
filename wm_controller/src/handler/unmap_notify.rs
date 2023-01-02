use std::error::Error;
use x11rb::protocol::xproto::UnmapNotifyEvent;

use super::WmHandler;

impl<'a> WmHandler<'a> {
    pub fn handle_unmap_notify(&mut self, event: &UnmapNotifyEvent) -> Result<(), Box<dyn Error>> {
        self.client_usecase.remove_client(event.window)
    }
}
