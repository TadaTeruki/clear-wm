use std::error::Error;
use x11rb::protocol::xproto::MotionNotifyEvent;
use zym_session::common::SessionImpl;

pub fn handle_motion_notify(
    _session: &dyn SessionImpl,
    _event: &MotionNotifyEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
