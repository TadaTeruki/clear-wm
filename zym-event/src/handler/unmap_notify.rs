use std::error::Error;
use x11rb::{connection::Connection, protocol::xproto::UnmapNotifyEvent};
use zym_domain::repository::session::SessionRepository;

pub struct UnmapNotifyHandler<'a, C: Connection> {
    session: &'a dyn SessionRepository<'a, C>,
    event: &'a UnmapNotifyEvent,
}

impl<'a, C: Connection> UnmapNotifyHandler<'a, C> {
    pub fn new(ss: &'a dyn SessionRepository<'a, C>, ev: &'a UnmapNotifyEvent) -> Self {
        Self {
            session: ss,
            event: ev,
        }
    }
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
