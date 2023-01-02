use std::error::Error;

use log::warn;
use wm_model::entity::client::ClientID;
use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt, StackMode};

use crate::client_manager::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn raise_client(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        let client = {
            if let Some(client_) = self.client_container.get(&client_id) {
                client_
            } else {
                warn!("client not found");
                return Ok(());
            }
        };

        self.connection.configure_window(
            client.frame,
            &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
        )?;

        Ok(())
    }
}
