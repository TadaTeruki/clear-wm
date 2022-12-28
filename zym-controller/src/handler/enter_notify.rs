use std::error::Error;
use x11rb::protocol::xproto::EnterNotifyEvent;
use zym_model::common::session::SessionImpl;

pub fn handle_enter_notify(
    _session: &dyn SessionImpl,
    _event: &EnterNotifyEvent,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
