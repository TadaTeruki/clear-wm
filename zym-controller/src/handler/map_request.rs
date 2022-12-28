use std::error::Error;
use x11rb::protocol::xproto::{ConnectionExt, MapRequestEvent};
use zym_model::common::session::SessionImpl;

pub fn handle_map_request(
    session: &dyn SessionImpl,
    event: &MapRequestEvent,
) -> Result<(), Box<dyn Error>> {
    session.connection().map_window(event.window)?;
    Ok(())
}
