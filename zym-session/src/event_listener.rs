use std::error::Error;

use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::xcb_ffi::XCBConnection;

use x11rb::errors::ReplyError;

use crate::common::EventListenerImpl;

pub struct WmEventListener<'a> {
    connection: &'a XCBConnection,
}

impl<'a> WmEventListener<'a> {
    pub fn new(connection_: &'a XCBConnection) -> Result<Self, ReplyError> {
        Ok(Self {
            connection: connection_,
        })
    }
}

impl<'a> EventListenerImpl<'a> for WmEventListener<'a> {
    fn wait(&self) -> Result<Event, Box<dyn Error>> {
        self.connection.flush()?;
        let event = self.connection.wait_for_event()?;
        Ok(event)
    }

    fn poll(&self) -> Result<Option<Event>, Box<dyn Error>> {
        let event = self.connection.poll_for_event()?;
        Ok(event)
    }
}
