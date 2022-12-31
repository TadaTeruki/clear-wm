use std::error::Error;

use x11rb::protocol::xproto::Window;

use crate::entity::{
    client::{ClientID, WindowType},
    geometry::Geometry,
};

pub trait ClientManagerImpl<'a> {
    fn create(&mut self, window: Window) -> Result<ClientID, Box<dyn Error>>;

    fn map(&self, client_id: ClientID) -> Result<(), Box<dyn Error>>;

    fn move_resize(
        &self,
        client_id: ClientID,
        geom: Geometry,
        window_type: WindowType,
    ) -> Result<(), Box<dyn Error>>;

    fn configure_window(&self, window: Window, geom: Geometry) -> Result<(), Box<dyn Error>>;

    fn query(&self, window: Window) -> Option<(ClientID, WindowType)>;
}
