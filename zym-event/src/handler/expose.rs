use std::error::Error;
use x11rb::{connection::Connection, protocol::xproto::ExposeEvent};
use zym_domain::repository::session::SessionRepository;

pub struct ExposeHandler<'a, C: Connection> {
    session: &'a dyn SessionRepository<'a, C>,
    event: &'a ExposeEvent,
}

impl<'a, C: Connection> ExposeHandler<'a, C> {
    pub fn new(ss: &'a dyn SessionRepository<'a, C>, ev: &'a ExposeEvent) -> Self {
        Self {
            session: ss,
            event: ev,
        }
    }
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
