use std::error::Error;

use x11rb::{
    protocol::xproto::{Screen, Window},
    xcb_ffi::XCBConnection,
};
use zym_config::WmConfig;

use crate::entity::{
    client::{geometry::ClientGeometry, ClientID, WindowType},
    visual::WmVisual,
};

pub trait ClientManagerImpl<'a> {
    fn create(
        &mut self,
        connection: &XCBConnection,
        screen: &Screen,
        visual: &WmVisual,
        config: &WmConfig,
        window: Window,
    ) -> Result<ClientID, Box<dyn Error>>;

    fn map(
        &self,
        connection: &XCBConnection,
        config: &WmConfig,
        client_id: ClientID,
    ) -> Result<(), Box<dyn Error>>;

    fn move_resize(
        &self,
        connection: &XCBConnection,
        config: &WmConfig,
        client_id: ClientID,
        geom: ClientGeometry,
    ) -> Result<(), Box<dyn Error>>;

    fn configure_window(
        &self,
        connection: &XCBConnection,
        window: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<(), Box<dyn Error>>;

    fn query(&self, window: Window) -> Option<(ClientID, WindowType)>;
}
