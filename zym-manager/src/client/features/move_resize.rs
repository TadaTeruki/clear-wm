use std::error::Error;

use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt};
use zym_model::entity::client::WmClient;

use crate::client::{geometry::ClientGeometry, manager::WmClientManager};

impl<'a> WmClientManager<'a> {
    pub fn move_resize_client(
        &self,
        client: &WmClient,
        geom: ClientGeometry,
    ) -> Result<(), Box<dyn Error>> {
        let frame_geom = geom.to_frame(self.config);

        self.connection.grab_server()?;
        self.connection.configure_window(
            client.frame,
            &ConfigureWindowAux::new()
                .x(frame_geom.x as i32)
                .y(frame_geom.y as i32)
                .width(frame_geom.width as u32)
                .height(frame_geom.height as u32),
        )?;

        let app_geom = geom.to_app_relative(self.config);

        self.connection.configure_window(
            client.app,
            &ConfigureWindowAux::new()
                .x(app_geom.x as i32)
                .y(app_geom.y as i32)
                .width(app_geom.width as u32)
                .height(app_geom.height as u32),
        )?;
        self.connection.ungrab_server()?;
        Ok(())
    }
}
