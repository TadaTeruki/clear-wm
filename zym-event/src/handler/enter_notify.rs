use std::error::Error;
use x11rb::{connection::Connection, protocol::xproto::EnterNotifyEvent};
use zym_domain::repository::session::SessionRepository;

pub struct EnterNotifyHandler<'a, C: Connection> {
    session: &'a dyn SessionRepository<'a, C>,
    event: &'a EnterNotifyEvent,
}

impl<'a, C: Connection> EnterNotifyHandler<'a, C> {
    pub fn new(ss: &'a dyn SessionRepository<'a, C>, ev: &'a EnterNotifyEvent) -> Self {
        Self {
            session: ss,
            event: ev,
        }
    }
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
