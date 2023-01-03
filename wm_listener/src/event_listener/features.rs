use std::error::Error;

use x11rb::{connection::Connection, protocol::Event};

use super::{traits::EventListenerImpl, types::WmEventListener};

impl<'a> EventListenerImpl<'a> for WmEventListener<'a> {
    fn wait(&self) -> Result<Event, Box<dyn Error>> {
        self.connection.flush()?;
        let event = self.connection.wait_for_event()?;
        Ok(event)
    }

    fn poll(&mut self) -> Result<Option<Event>, Box<dyn Error>> {
        loop {
            let event_option = self.connection.poll_for_event()?;
            if let Some(event) = event_option {
                self.cache(event.response_type(), event);
            } else {
                return Ok(self.cache.pop_front());
            }
        }
    }
}
