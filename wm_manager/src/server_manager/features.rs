use std::error::Error;

use wm_model::traits::server_manager::ServerManagerImpl;
use x11rb::{protocol::xproto::ConnectionExt, wrapper::ConnectionExt as _};

use super::types::WmServerManager;

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
