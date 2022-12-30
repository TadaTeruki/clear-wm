use std::error::Error;
use x11rb::protocol::xproto::EnterNotifyEvent;
use zym_session::common::ClientSessionImpl;

pub fn handle_enter_notify(
    _session: &dyn ClientSessionImpl,
    _event: &EnterNotifyEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
