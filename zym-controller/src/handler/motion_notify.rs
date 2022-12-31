use std::error::Error;
use x11rb::protocol::xproto::MotionNotifyEvent;

use super::WmHandler;

impl<'a> WmHandler<'a> {
    pub fn handle_motion_notify(&self, _event: &MotionNotifyEvent) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
