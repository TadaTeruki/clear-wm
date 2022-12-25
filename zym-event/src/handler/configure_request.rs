use log::info;
use std::error::Error;
use x11rb::{
    connection::Connection,
    protocol::xproto::{ConfigureRequestEvent, ConfigureWindowAux, ConnectionExt},
};
use zym_domain::repository::session::SessionRepository;

pub struct ConfigureRequestHandler<'a, C: Connection> {
    session: &'a dyn SessionRepository<'a, C>,
    event: &'a ConfigureRequestEvent,
}

impl<'a, C: Connection> ConfigureRequestHandler<'a, C> {
    pub fn new(ss: &'a dyn SessionRepository<'a, C>, ev: &'a ConfigureRequestEvent) -> Self {
        Self {
            session: ss,
            event: ev,
        }
    }
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        let aux = ConfigureWindowAux::from_configure_request(self.event)
            .sibling(None)
            .stack_mode(None);
        self.session
            .connection()
            .configure_window(self.event.window, &aux)?;
        info!("Configured: {:?}", aux);
        Ok(())
    }
}
