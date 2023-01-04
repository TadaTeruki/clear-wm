use std::error::Error;

use x11rb::protocol::xproto::Window;

use crate::entity::{
    client::{WindowType, WmClient},
    geometry::{Geometry, WmMoveResizeMask},
};

pub trait ClientManagerImpl<'a> {
    fn create(&mut self, window: Window) -> Result<WmClient, Box<dyn Error>>;

    fn map(&self, client: &WmClient) -> Result<(), Box<dyn Error>>;

    fn move_resize(
        &self,
        client: &WmClient,
        geom: Geometry,
        window_type: WindowType,
        mask: WmMoveResizeMask,
    ) -> Result<(), Box<dyn Error>>;

    fn get_geometry(
        &self,
        client: &WmClient,
        window_type: WindowType,
    ) -> Result<Option<Geometry>, Box<dyn Error>>;

    fn draw_frame(&self, client: &WmClient, client_title: String) -> Result<(), Box<dyn Error>>;

    fn remove(&mut self, client: &WmClient) -> Result<(), Box<dyn Error>>;

    fn set_focus(&self, client: &WmClient) -> Result<(), Box<dyn Error>>;

    fn get_focus(&self) -> Result<Window, Box<dyn Error>>;

    fn raise(&self, client: &WmClient) -> Result<(), Box<dyn Error>>;

    fn grab_button(&self, client: &WmClient) -> Result<(), Box<dyn Error>>;

    fn ungrab_button(&self, client: &WmClient) -> Result<(), Box<dyn Error>>;
}
