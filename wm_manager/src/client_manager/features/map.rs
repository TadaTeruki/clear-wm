use std::error::Error;

use wm_model::entity::client::WmClient;
use x11rb::protocol::xproto::{ConnectionExt, SetMode};

use crate::client_manager::types::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn map_client(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.connection
            .change_save_set(SetMode::INSERT, client.app)?;
        self.connection.map_window(client.frame)?;
        self.connection.map_window(client.app)?;
        Ok(())
    }
}
