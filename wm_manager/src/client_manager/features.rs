mod create;
mod draw;
mod focus;
mod get_geometry;
mod grab;
mod map;
mod move_resize;
mod raise;
mod remove;

use std::error::Error;

use wm_model::{
    entity::{
        client::{ClientID, WindowType},
        geometry::Geometry,
    },
    traits::client_manager::ClientManagerImpl,
};
use x11rb::protocol::xproto::Window;

use self::move_resize::WmMoveResizeMask;

use super::types::manager::WmClientManager;

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

    fn query_id(&self, window: Window) -> Option<(ClientID, WindowType)> {
        self.client_index.get(&window).copied()
    }

    fn draw_frame(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        self.draw_client_frame(client_id)
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
