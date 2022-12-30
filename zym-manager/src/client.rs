pub mod create;
pub mod map;
pub mod move_resize;

use std::error::Error;

use x11rb::{
    protocol::xproto::{ConfigureWindowAux, ConnectionExt, Screen, Window},
    xcb_ffi::XCBConnection,
};
use zym_config::WmConfig;
use zym_model::{
    common::manager::ClientManagerImpl,
    entity::{
        client::{geometry::ClientGeometry, ClientID, WindowType},
        visual::WmVisual,
    },
};

use crate::manager::WmClientManager;

impl<'a> ClientManagerImpl<'a> for WmClientManager {
    fn create(
        &mut self,
        connection: &XCBConnection,
        screen: &Screen,
        visual: &WmVisual,
        config: &WmConfig,
        window: Window,
    ) -> Result<ClientID, Box<dyn Error>> {
        self.create_client(connection, screen, visual, config, window)
    }

    fn map(
        &self,
        connection: &XCBConnection,
        config: &WmConfig,
        client_id: ClientID,
    ) -> Result<(), Box<dyn Error>> {
        self.map_client(connection, config, client_id)
    }

    fn move_resize(
        &self,
        connection: &XCBConnection,
        config: &WmConfig,
        client_id: ClientID,
        geom: ClientGeometry,
    ) -> Result<(), Box<dyn Error>> {
        self.move_resize_client(connection, config, client_id, geom)
    }

    fn configure_window(
        &self,
        connection: &XCBConnection,
        window: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<(), Box<dyn Error>> {
        let aux = ConfigureWindowAux::new()
            .x(x as i32)
            .y(y as i32)
            .width(width as u32)
            .height(height as u32)
            .sibling(None)
            .stack_mode(None);
        connection.configure_window(window, &aux)?;
        Ok(())
    }

    fn query(&self, window: Window) -> Option<(ClientID, WindowType)> {
        self.client_index.get(&window).copied()
    }
}
