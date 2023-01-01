use std::error::Error;

use x11rb::protocol::xproto::{ConnectionExt, SetMode};
use zym_model::entity::client::WmClient;

use crate::client::{geometry::app_relative_position, manager::WmClientManager};

impl<'a> WmClientManager<'a> {
    pub fn map_client(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.connection.grab_server()?;
        self.connection
            .change_save_set(SetMode::INSERT, client.app)?;

        let (rx, ry) = app_relative_position(self.config);

        self.connection
            .reparent_window(client.app, client.frame, rx, ry)?;
        self.connection.map_window(client.frame)?;
        self.connection.map_window(client.app)?;
        self.connection.ungrab_server()?;
        Ok(())
    }
}
