use std::error::Error;
use x11rb::protocol::xproto::ButtonReleaseEvent;

use crate::event_handler::types::WmEventHandler;

impl<'a> WmEventHandler<'a> {
    pub fn handle_button_release(
        &mut self,
        _event: &ButtonReleaseEvent,
    ) -> Result<(), Box<dyn Error>> {
        self.client_usecase.release_dragging_client()
    }
}
