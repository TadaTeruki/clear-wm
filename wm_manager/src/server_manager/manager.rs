use x11rb::xcb_ffi::XCBConnection;

pub struct WmServerManager<'a> {
    pub(super) connection: &'a XCBConnection,
}

impl<'a> WmServerManager<'a> {
    pub fn new(connection_: &'a XCBConnection) -> Self {
        Self {
            connection: connection_,
        }
    }
}
