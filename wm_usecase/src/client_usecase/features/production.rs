use std::error::Error;

use log::{error, warn};
use x11rb::protocol::xproto::Window;

use crate::client_usecase::types::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn usecase_compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        let res = self
            .collection_manager
            .push(self.client_manager.create(window)?)?;

        if let Some(client) = res {
            self.server_manager.grab()?;
            self.client_manager.map(client)?;
            self.client_manager
                .draw_frame(client, self.property_manager.client_title(client)?)?;
            self.server_manager.sync()?;
            self.server_manager.ungrab()?;
        } else {
            error!("failed to setup new client");
        }

        Ok(())
    }

    pub fn usecase_remove_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        if let Some(client) = self.collection_manager.query_from_window(window)? {
            self.client_manager.remove(client)?;
            self.collection_manager.remove(window)?;
        } else {
            warn!("client not found");
        }

        Ok(())
    }
}
