use std::error::Error;
use x11rb::protocol::xproto::UnmapNotifyEvent;
use zym_session::common::ClientSessionImpl;
pub fn handle_unmap_notify(
    _session: &dyn ClientSessionImpl,
    _event: &UnmapNotifyEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
