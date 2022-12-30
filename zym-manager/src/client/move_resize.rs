use std::error::Error;

use log::warn;
use x11rb::{
    protocol::xproto::{ConfigureWindowAux, ConnectionExt},
    xcb_ffi::XCBConnection,
};
use zym_config::WmConfig;
use zym_model::entity::client::{geometry::ClientGeometry, ClientID};

use crate::manager::WmClientManager;

impl WmClientManager {
    pub fn move_resize_client(
        &self,
        connection: &XCBConnection,
        config: &WmConfig,
        client_id: ClientID,
        geom: ClientGeometry,
    ) -> Result<(), Box<dyn Error>> {
        let client_option = self.client_container.get(&client_id);

        if let Some(client) = client_option {
            let frame_geom = geom.to_frame(config);
            connection.configure_window(
                client.frame,
                &ConfigureWindowAux::new()
                    .x(frame_geom.x as i32)
                    .y(frame_geom.y as i32)
                    .width(frame_geom.width as u32)
                    .height(frame_geom.height as u32),
            )?;

            let app_geom = geom.to_app_relative(config);

            connection.configure_window(
                client.app,
                &ConfigureWindowAux::new()
                    .x(app_geom.x as i32)
                    .y(app_geom.y as i32)
                    .width(app_geom.width as u32)
                    .height(app_geom.height as u32),
            )?;
        } else {
            warn!("client not found");
        }
        Ok(())
    }
}
