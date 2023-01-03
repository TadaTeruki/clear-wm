mod create;
mod draw;
mod focus;
mod get_geometry;
mod grab_button;
mod map;
mod move_resize;
mod raise;
mod remove;

use std::error::Error;

use wm_model::{
    entity::{
        client::{WindowType, WmClient},
        geometry::{Geometry, WmMoveResizeMask},
    },
    traits::client_manager::ClientManagerImpl,
};
use x11rb::protocol::xproto::Window;

use super::types::manager::WmClientManager;

impl<'a> ClientManagerImpl<'a> for WmClientManager<'a> {
    fn create(&mut self, window: Window) -> Result<WmClient, Box<dyn Error>> {
        self.create_client(window)
    }

    fn map(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.map_client(client)
    }

    fn get_geometry(
        &self,
        client: &WmClient,
        window_type: WindowType,
    ) -> Result<Option<Geometry>, Box<dyn Error>> {
        self.get_client_geometry(client, window_type)
    }

    fn move_resize(
        &self,
        client: &WmClient,
        geom: Geometry,
        window_type: WindowType,
        mask: WmMoveResizeMask,
    ) -> Result<(), Box<dyn Error>> {
        self.move_resize_client(client, geom, window_type, mask)
    }

    fn draw_frame(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.draw_client_frame(client)
    }

    fn remove(&mut self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.remove_client(client)
    }

    fn set_focus(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.set_client_focus(client)
    }

    fn get_focus(&self) -> Result<Window, Box<dyn Error>> {
        self.get_client_focus()
    }

    fn raise(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.raise_client(client)
    }

    fn grab_button(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.grab_button_client(client)
    }

    fn ungrab_button(&self, client: &WmClient) -> Result<(), Box<dyn Error>> {
        self.ungrab_button_client(client)
    }
}
