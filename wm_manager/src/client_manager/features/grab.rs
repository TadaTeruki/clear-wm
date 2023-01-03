use std::error::Error;

use wm_model::entity::client::WmClient;
use x11rb::protocol::xproto::{
    ButtonIndex, ConnectionExt, EventMask, GrabMode, ModMask, WindowEnum,
};

use crate::client_manager::types::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn grab_client(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
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

    pub fn ungrab_client(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.connection
            .ungrab_button(ButtonIndex::ANY, client.app, ModMask::ANY)?;
        Ok(())
    }
}
