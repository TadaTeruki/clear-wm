use std::error::Error;

use log::warn;
use x11rb::protocol::xproto::Window;
use zym_model::entity::client::WindowType;

use crate::client::usecase::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        let client_id = self.client_manager.create(window)?;
        self.server_manager.grab()?;
        self.client_manager.map(client_id)?;
        self.client_manager.draw_frame(client_id)?;
        self.server_manager.sync()?;
        self.server_manager.ungrab()?;
        Ok(())
    }

    pub fn remove_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        if let Some((client_id, WindowType::ComposedApp)) = self.client_manager.query_id(window) {
            self.client_manager.remove(client_id)?
        } else {
            warn!("window not managed");
        }
        Ok(())
    }
}
