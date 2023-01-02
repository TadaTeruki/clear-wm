use std::error::Error;

use log::warn;
use wm_model::entity::client::ClientID;
use x11rb::{
    protocol::xproto::{ConnectionExt, InputFocus},
    CURRENT_TIME,
};

use crate::client_manager::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn set_client_focus(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        let client = {
            if let Some(client_) = self.client_container.get(&client_id) {
                client_
            } else {
                warn!("client not found");
                return Ok(());
            }
        };
        self.connection
            .set_input_focus(InputFocus::POINTER_ROOT, client.app, CURRENT_TIME)?;
        Ok(())
    }

    pub fn get_client_focus(&self) -> Result<Option<ClientID>, Box<dyn Error>> {
        let focus = self.connection.get_input_focus()?.reply()?;

        if let Some((client_id, _)) = self.client_index.get(&focus.focus) {
            return Ok(Some(*client_id));
        }
        Ok(None)
    }
}
