use std::error::Error;
use x11rb::{
    connection::Connection,
    protocol::xproto::{ConnectionExt, MapRequestEvent},
};
use zym_domain::repository::session::SessionRepository;

pub struct MapRequestHandler<'a, C: Connection> {
    session: &'a dyn SessionRepository<'a, C>,
    event: &'a MapRequestEvent,
}

impl<'a, C: Connection> MapRequestHandler<'a, C> {
    pub fn new(ss: &'a dyn SessionRepository<'a, C>, ev: &'a MapRequestEvent) -> Self {
        Self {
            session: ss,
            event: ev,
        }
    }
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        self.session.connection().map_window(self.event.window)?;
        Ok(())
    }
}
