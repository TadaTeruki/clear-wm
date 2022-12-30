use std::error::Error;

use crate::common::EventSessionImpl;
use x11rb::{connection::Connection, protocol::Event};

use super::WmSession;

impl<'a> EventSessionImpl<'a> for WmSession<'a> {
    fn wait_for_event(&self) -> Result<Event, Box<dyn Error>> {
        self.connection.flush()?;
        let event = self.connection.wait_for_event()?;
        Ok(event)
    }

    fn poll_for_event(&self) -> Result<Option<Event>, Box<dyn Error>> {
        let event = self.connection.poll_for_event()?;
        Ok(event)
    }
}
