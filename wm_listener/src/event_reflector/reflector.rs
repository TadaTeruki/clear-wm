use x11rb::xcb_ffi::XCBConnection;

use x11rb::errors::ReplyError;

pub struct WmEventReflector<'a> {
    pub(super) connection: &'a XCBConnection,
}

impl<'a> WmEventReflector<'a> {
    pub fn new(connection_: &'a XCBConnection) -> Result<Self, ReplyError> {
        Ok(Self {
            connection: connection_,
        })
    }
}
