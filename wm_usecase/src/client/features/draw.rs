use std::error::Error;

use log::warn;
use wm_model::entity::client::WindowType;
use x11rb::protocol::xproto::Window;

use crate::client::usecase::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn draw_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        if let Some((client_id, WindowType::Frame)) = self.client_manager.query_id(window) {
            self.client_manager.draw_frame(client_id)?;
        } else {
            warn!("window not drawable");
        }
        Ok(())
    }
}
