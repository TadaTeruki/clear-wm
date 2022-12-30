use std::error::Error;
use x11rb::protocol::xproto::MotionNotifyEvent;
use zym_session::common::ClientSessionImpl;

pub fn handle_motion_notify(
    _session: &dyn ClientSessionImpl,
    _event: &MotionNotifyEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
