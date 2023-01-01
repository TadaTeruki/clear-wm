use std::error::Error;

use log::warn;
use x11rb::protocol::xproto::Window;
use zym_model::entity::client::WindowType;

use crate::client::usecase::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        let client_id = self.client_manager.create(window)?;
        self.client_manager.map(client_id)?;
        Ok(())
    }

    pub fn remove_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        if let Some((client_id, WindowType::ComposedApp)) = self.client_manager.query(window) {
            self.client_manager.remove(client_id)?
        } else {
            warn!("window not managed");
        }
        Ok(())
    }
}
