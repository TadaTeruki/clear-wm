use std::error::Error;

use log::warn;
use x11rb::protocol::xproto::Window;

use crate::client_usecase::types::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn usecase_draw_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        if let Some(client) = self.collection_manager.query_from_window(window)? {
            self.client_manager
                .draw_frame(client, self.property_manager.client_title(client)?)?;
        } else {
            warn!("client not found");
        }

        Ok(())
    }
}
