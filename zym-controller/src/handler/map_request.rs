use std::error::Error;
use x11rb::protocol::xproto::MapRequestEvent;
use zym_session::common::SessionImpl;

pub fn handle_map_request(
    session: &mut dyn SessionImpl,
    event: &MapRequestEvent,
) -> Result<(), Box<dyn Error>> {
    session.compose_client(event.window)?;
    Ok(())
}
