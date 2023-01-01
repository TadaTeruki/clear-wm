pub mod create;
pub mod map;
pub mod move_resize;

use std::error::Error;

use log::warn;
use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt, Window};
use zym_model::{
    common::manager::ClientManagerImpl,
    entity::{
        client::{ClientID, WindowType},
        geometry::Geometry,
    },
};

use super::{geometry::ClientGeometry, manager::WmClientManager};

impl<'a> ClientManagerImpl<'a> for WmClientManager<'a> {
    fn create(&mut self, window: Window) -> Result<ClientID, Box<dyn Error>> {
        self.create_client(window)
    }

    fn map(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        if let Some(client) = self.client_container.get(&client_id) {
            self.map_client(client)?
        } else {
            warn!("client not found");
        }
        Ok(())
    }

    fn move_resize(
        &self,
        client_id: ClientID,
        geom: Geometry,
        window_type: WindowType,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(client) = self.client_container.get(&client_id) {
            match window_type {
                WindowType::Frame => {
                    self.move_resize_client(client, ClientGeometry::from_frame(geom, self.config))?;
                }
                _ => {
                    warn!(
                        "unable client to move or resize. (window type: {:?})",
                        window_type
                    );
                }
            }
        } else {
            warn!("client not found");
        }
        Ok(())
    }

    fn configure_window(&self, window: Window, geom: Geometry) -> Result<(), Box<dyn Error>> {
        let aux = ConfigureWindowAux::new()
            .x(geom.x as i32)
            .y(geom.y as i32)
            .width(geom.width as u32)
            .height(geom.height as u32)
            .sibling(None)
            .stack_mode(None);
        self.connection.configure_window(window, &aux)?;
        Ok(())
    }

    fn query(&self, window: Window) -> Option<(ClientID, WindowType)> {
        self.client_index.get(&window).copied()
    }
}
