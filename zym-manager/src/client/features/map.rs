use std::error::Error;

use log::warn;
use x11rb::protocol::xproto::{ConnectionExt, SetMode};
use zym_model::entity::client::ClientID;

use crate::client::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn map_client(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        let client = {
            if let Some(client_) = self.client_container.get(&client_id) {
                client_
            } else {
                warn!("client not found");
                return Ok(());
            }
        };

        self.connection
            .change_save_set(SetMode::INSERT, client.app)?;
        self.connection.map_window(client.frame)?;
        self.connection.map_window(client.app)?;
        Ok(())
    }
}
