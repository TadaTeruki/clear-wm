use std::error::Error;

use wm_model::entity::{
    client::{WindowType, WmClient},
    geometry::{Geometry, WmMoveResizeMask},
};
use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt};

use crate::client_manager::types::{geometry::ClientGeometry, manager::WmClientManager};

impl<'a> WmClientManager<'a> {
    pub fn move_resize_client(
        &self,
        client: &WmClient,
        geom: Geometry,
        window_type: WindowType,
        mask: WmMoveResizeMask,
    ) -> Result<(), Box<dyn Error>> {
        let client_geom = match window_type {
            WindowType::Frame => ClientGeometry::from_frame(geom, self.config).fix_position(),
            _ => {
                return Ok(());
            }
        };

        let frame_geom = client_geom.to_frame(self.config).for_system_api()?;
        let app_geom = client_geom.to_app_relative(self.config).for_system_api()?;

        match mask {
            WmMoveResizeMask::MoveResize => {
                self.connection.configure_window(
                    client.frame,
                    &ConfigureWindowAux::new()
                        .x(frame_geom.x as i32)
                        .y(frame_geom.y as i32)
                        .width(frame_geom.width as u32)
                        .height(frame_geom.height as u32),
                )?;
                self.connection.configure_window(
                    client.app,
                    &ConfigureWindowAux::new()
                        .width(app_geom.width as u32)
                        .height(app_geom.height as u32),
                )?;

                client
                    .frame_surface
                    .set_size(frame_geom.width as i32, frame_geom.height as i32)?;
            }
            WmMoveResizeMask::Move => {
                self.connection.configure_window(
                    client.frame,
                    &ConfigureWindowAux::new()
                        .x(frame_geom.x as i32)
                        .y(frame_geom.y as i32),
                )?;
            }
            WmMoveResizeMask::Resize => {
                self.connection.configure_window(
                    client.frame,
                    &ConfigureWindowAux::new()
                        .width(frame_geom.width as u32)
                        .height(frame_geom.height as u32),
                )?;
                self.connection.configure_window(
                    client.app,
                    &ConfigureWindowAux::new()
                        .width(app_geom.width as u32)
                        .height(app_geom.height as u32),
                )?;

                client
                    .frame_surface
                    .set_size(frame_geom.width as i32, frame_geom.height as i32)?;
            }
        }

        Ok(())
    }
}
