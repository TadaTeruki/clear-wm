use std::error::Error;
use x11rb::{connection::Connection, protocol::xproto::ButtonPressEvent};
use zym_domain::repository::session::SessionRepository;

pub struct ButtonPressHandler<'a, C: Connection> {
    session: &'a dyn SessionRepository<'a, C>,
    event: &'a ButtonPressEvent,
}

impl<'a, C: Connection> ButtonPressHandler<'a, C> {
    pub fn new(ss: &'a dyn SessionRepository<'a, C>, ev: &'a ButtonPressEvent) -> Self {
        Self {
            session: ss,
            event: ev,
        }
    }
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
