use std::error::Error;

use x11rb::{
    protocol::xproto::{ConnectionExt, SetMode},
    xcb_ffi::XCBConnection,
};
use zym_config::WmConfig;
use zym_model::entity::client::ClientID;

use crate::manager::WmClientManager;

impl WmClientManager {
    pub fn map_client(
        &self,
        connection: &XCBConnection,
        client_id: ClientID,
        config: &WmConfig,
    ) -> Result<(), Box<dyn Error>> {
        let client_option = self.client_container.get(&client_id);
        let client_config = &config.client;

        if let Some(client) = client_option {
            let border_width = client_config.frame.border_width as i16;
            let titlebar_height = client_config.frame.titlebar_height as i16;
            connection.grab_server()?;
            connection.change_save_set(SetMode::INSERT, client.window)?;
            connection.reparent_window(
                client.window,
                client.frame,
                border_width,
                border_width + titlebar_height,
            )?;
            connection.map_window(client.frame)?;
            connection.map_window(client.window)?;
            connection.ungrab_server()?;
        }

        Ok(())
    }
}
