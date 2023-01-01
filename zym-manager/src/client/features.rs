pub mod create;
pub mod focus;
pub mod get_geometry;
pub mod grab;
pub mod map;
pub mod move_resize;
pub mod raise;
pub mod remove;

use std::error::Error;

use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt, Window};
use zym_model::{
    common::manager::ClientManagerImpl,
    entity::{
        client::{ClientID, WindowType},
        geometry::Geometry,
    },
};

use self::move_resize::WmMoveResizeMask;

use super::manager::WmClientManager;

impl<'a> ClientManagerImpl<'a> for WmClientManager<'a> {
    fn create(&mut self, window: Window) -> Result<ClientID, Box<dyn Error>> {
        self.create_client(window)
    }

    fn map(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        self.map_client(client_id)
    }

    fn get_geometry(
        &self,
        client_id: ClientID,
        window_type: WindowType,
    ) -> Result<Option<Geometry>, Box<dyn Error>> {
        self.get_client_geometry(client_id, window_type)
    }

    fn move_resize(
        &self,
        client_id: ClientID,
        geom: Geometry,
        window_type: WindowType,
    ) -> Result<(), Box<dyn Error>> {
        self.move_resize_client(client_id, geom, window_type, WmMoveResizeMask::MoveResize)
    }

    fn move_to(
        &self,
        client_id: ClientID,
        x_: i32,
        y_: i32,
        window_type: WindowType,
    ) -> Result<(), Box<dyn Error>> {
        self.move_resize_client(
            client_id,
            Geometry {
                x: x_,
                y: y_,
                width: 0,
                height: 0,
            },
            window_type,
            WmMoveResizeMask::Move,
        )
    }

    fn resize(
        &self,
        client_id: ClientID,
        width_: i32,
        height_: i32,
        window_type: WindowType,
    ) -> Result<(), Box<dyn Error>> {
        self.move_resize_client(
            client_id,
            Geometry {
                x: 0,
                y: 0,
                width: width_,
                height: height_,
            },
            window_type,
            WmMoveResizeMask::Resize,
        )
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

    fn remove(&mut self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        self.remove_client(client_id)
    }

    fn set_focus(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        self.set_client_focus(client_id)
    }

    fn get_focus(&self) -> Result<Option<ClientID>, Box<dyn Error>> {
        self.get_client_focus()
    }

    fn raise(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        self.raise_client(client_id)
    }

    fn grab(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        self.grab_client(client_id)
    }

    fn ungrab(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        self.ungrab_client(client_id)
    }
}
