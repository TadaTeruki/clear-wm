use std::error::Error;

use log::warn;
use wm_model::entity::client::ClientID;
use x11rb::protocol::xproto::{
    ButtonIndex, ConnectionExt, EventMask, GrabMode, ModMask, WindowEnum,
};

use crate::client_manager::types::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn grab_client(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        let client = {
            if let Some(client_) = self.client_container.get(&client_id) {
                client_
            } else {
                warn!("client not found");
                return Ok(());
            }
        };
        self.connection.grab_button(
            true,
            client.app,
            EventMask::BUTTON_PRESS | EventMask::BUTTON_RELEASE,
            GrabMode::ASYNC,
            GrabMode::ASYNC,
            WindowEnum::NONE,
            WindowEnum::NONE,
            ButtonIndex::ANY,
            ModMask::ANY,
        )?;
        Ok(())
    }

    pub fn ungrab_client(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        let client = {
            if let Some(client_) = self.client_container.get(&client_id) {
                client_
            } else {
                warn!("client not found");
                return Ok(());
            }
        };
        self.connection
            .ungrab_button(ButtonIndex::ANY, client.app, ModMask::ANY)?;
        Ok(())
    }
}
