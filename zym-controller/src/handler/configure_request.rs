use log::info;
use std::error::Error;
use x11rb::protocol::xproto::{ConfigureRequestEvent, ConfigureWindowAux, ConnectionExt};
use zym_model::common::session::SessionImpl;

pub fn handle_configure_request(
    session: &dyn SessionImpl,
    event: &ConfigureRequestEvent,
) -> Result<(), Box<dyn Error>> {
    let aux = ConfigureWindowAux::from_configure_request(event)
        .sibling(None)
        .stack_mode(None);
    session.connection().configure_window(event.window, &aux)?;
    info!("Configured: {:?}", aux);
    Ok(())
}
