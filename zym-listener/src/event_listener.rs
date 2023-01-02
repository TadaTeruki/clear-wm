use std::collections::VecDeque;
use std::error::Error;

use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::xcb_ffi::XCBConnection;

use x11rb::errors::ReplyError;

use crate::common::EventListenerImpl;

pub struct WmEventListener<'a> {
    connection: &'a XCBConnection,
    cache: VecDeque<Event>,
}

impl<'a> WmEventListener<'a> {
    pub fn new(connection_: &'a XCBConnection) -> Result<Self, ReplyError> {
        Ok(Self {
            connection: connection_,
            cache: VecDeque::new(),
        })
    }

    fn cache(&mut self, response_type: u8, event: Event) {
        self.cache.retain(|ex| ex.response_type() != response_type);
        self.cache.push_back(event);
    }
}

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
