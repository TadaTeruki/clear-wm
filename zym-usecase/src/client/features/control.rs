use std::error::Error;

use log::warn;
use x11rb::protocol::xproto::Window;

use crate::client::usecase::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn activate_client(&self, window: Window) -> Result<(), Box<dyn Error>> {
        if let Some((client_id, _)) = self.client_manager.query(window) {
            self.client_manager.set_focus(client_id)?;
            self.client_manager.raise(client_id)?;
        } else {
            warn!("window not managed");
        }
        Ok(())
    }
}
