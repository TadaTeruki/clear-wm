use std::collections::VecDeque;
use x11rb::protocol::Event;
use x11rb::xcb_ffi::XCBConnection;

use x11rb::errors::ReplyError;

pub struct WmEventListener<'a> {
    pub(super) connection: &'a XCBConnection,
    pub(super) cache: VecDeque<Event>,
}

impl<'a> WmEventListener<'a> {
    pub fn new(connection_: &'a XCBConnection) -> Result<Self, ReplyError> {
        Ok(Self {
            connection: connection_,
            cache: VecDeque::new(),
        })
    }

    pub(super) fn cache(&mut self, response_type: u8, event: Event) {
        self.cache.retain(|ex| ex.response_type() != response_type);
        self.cache.push_back(event);
    }
}
