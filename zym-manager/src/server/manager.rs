use std::error::Error;

use x11rb::{protocol::xproto::ConnectionExt, wrapper::ConnectionExt as _, xcb_ffi::XCBConnection};
use zym_model::common::manager::ServerManagerImpl;

pub struct WmServerManager<'a> {
    pub connection: &'a XCBConnection,
}

impl<'a> WmServerManager<'a> {
    pub fn new(connection_: &'a XCBConnection) -> Self {
        Self {
            connection: connection_,
        }
    }
}

impl<'a> ServerManagerImpl for WmServerManager<'a> {
    fn grab(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection.grab_server()?;
        Ok(())
    }
    fn ungrab(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection.ungrab_server()?;
        Ok(())
    }
    fn sync(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection.sync()?;
        Ok(())
    }
}
