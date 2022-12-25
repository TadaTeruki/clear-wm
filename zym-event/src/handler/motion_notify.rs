use std::error::Error;
use x11rb::{connection::Connection, protocol::xproto::MotionNotifyEvent};
use zym_domain::repository::session::SessionRepository;

pub struct MotionNotifyHandler<'a, C: Connection> {
    session: &'a dyn SessionRepository<'a, C>,
    event: &'a MotionNotifyEvent,
}

impl<'a, C: Connection> MotionNotifyHandler<'a, C> {
    pub fn new(ss: &'a dyn SessionRepository<'a, C>, ev: &'a MotionNotifyEvent) -> Self {
        Self {
            session: ss,
            event: ev,
        }
    }
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
