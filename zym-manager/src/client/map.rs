use std::error::Error;

use x11rb::{
    protocol::xproto::{ConnectionExt, SetMode},
    xcb_ffi::XCBConnection,
};
use zym_config::WmConfig;
use zym_model::entity::client::{geometry::app_relative_position, ClientID};

use crate::manager::WmClientManager;

impl WmClientManager {
    pub fn map_client(
        &self,
        connection: &XCBConnection,
        config: &WmConfig,
        client_id: ClientID,
    ) -> Result<(), Box<dyn Error>> {
        let client_option = self.client_container.get(&client_id);

        if let Some(client) = client_option {
            connection.grab_server()?;
            connection.change_save_set(SetMode::INSERT, client.app)?;

            let (rx, ry) = app_relative_position(config);

            connection.reparent_window(client.app, client.frame, rx, ry)?;
            connection.map_window(client.frame)?;
            connection.map_window(client.app)?;
            connection.ungrab_server()?;
        }

        Ok(())
    }
}
