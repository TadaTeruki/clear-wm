use std::error::Error;
use x11rb::protocol::xproto::UnmapNotifyEvent;

use crate::event_handler::types::WmEventHandler;

impl<'a> WmEventHandler<'a> {
    pub fn handle_unmap_notify(&mut self, event: &UnmapNotifyEvent) -> Result<(), Box<dyn Error>> {
        self.client_usecase.remove_client(event.window)
    }
}
