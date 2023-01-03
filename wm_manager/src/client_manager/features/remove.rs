use std::error::Error;

use wm_model::entity::client::WmClient;
use x11rb::protocol::xproto::ConnectionExt;

use crate::client_manager::types::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn remove_client(&mut self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.connection.unmap_window(client.frame)?;
        self.connection.destroy_window(client.frame)?;

        Ok(())
    }
}
