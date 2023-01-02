use std::error::Error;
use x11rb::protocol::xproto::MotionNotifyEvent;

use crate::event_handler::handler::WmEventHandler;

impl<'a> WmEventHandler<'a> {
    pub fn handle_motion_notify(
        &mut self,
        event: &MotionNotifyEvent,
    ) -> Result<(), Box<dyn Error>> {
        self.client_usecase
            .drag_client(event.root_x.into(), event.root_y.into())
    }
}
