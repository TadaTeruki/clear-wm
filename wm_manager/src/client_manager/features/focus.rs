use std::error::Error;

use wm_model::entity::client::WmClient;
use x11rb::{
    protocol::xproto::{ConnectionExt, InputFocus, Window},
    CURRENT_TIME,
};

use crate::client_manager::types::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn set_client_focus(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.connection
            .set_input_focus(InputFocus::POINTER_ROOT, client.app, CURRENT_TIME)?;
        Ok(())
    }

    pub fn get_client_focus(&self) -> Result<Window, Box<dyn Error>> {
        let focus = self.connection.get_input_focus()?.reply()?;

        Ok(focus.focus)
    }
}
