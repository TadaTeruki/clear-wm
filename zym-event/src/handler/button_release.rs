use std::error::Error;
use x11rb::{connection::Connection, protocol::xproto::ButtonReleaseEvent};
use zym_domain::repository::session::SessionRepository;

pub struct ButtonReleaseHandler<'a, C: Connection> {
    session: &'a dyn SessionRepository<'a, C>,
    event: &'a ButtonReleaseEvent,
}

impl<'a, C: Connection> ButtonReleaseHandler<'a, C> {
    pub fn new(ss: &'a dyn SessionRepository<'a, C>, ev: &'a ButtonReleaseEvent) -> Self {
        Self {
            session: ss,
            event: ev,
        }
    }
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
