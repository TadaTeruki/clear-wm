use std::error::Error;

use log::warn;
use wm_model::entity::client::ClientID;
use x11rb::protocol::xproto::ConnectionExt;

use crate::client_manager::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn remove_client(&mut self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        let client = {
            if let Some(client_) = self.client_container.get(&client_id) {
                client_
            } else {
                warn!("client not found");
                return Ok(());
            }
        };

        self.connection.unmap_window(client.frame)?;
        self.connection.destroy_window(client.frame)?;

        self.client_index.remove(&client.app);
        self.client_index.remove(&client.frame);
        self.client_container.remove(&client_id);

        Ok(())
    }
}
