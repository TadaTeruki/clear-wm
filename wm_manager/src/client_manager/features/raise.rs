use std::error::Error;

use wm_model::entity::client::WmClient;
use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt, StackMode};

use crate::client_manager::types::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn raise_client(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.connection.configure_window(
            client.frame,
            &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
        )?;

        Ok(())
    }
}
