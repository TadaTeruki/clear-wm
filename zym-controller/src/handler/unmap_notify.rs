use std::error::Error;
use x11rb::protocol::xproto::UnmapNotifyEvent;
use zym_model::common::session::SessionImpl;

pub fn handle_unmap_notify(
    _session: &dyn SessionImpl,
    _event: &UnmapNotifyEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
