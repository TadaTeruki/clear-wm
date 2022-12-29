use std::error::Error;

use x11rb::{
    protocol::xproto::{ConfigureRequestEvent, Screen, Window},
    xcb_ffi::XCBConnection,
};
use zym_config::WmConfig;

use crate::entity::{client::ClientID, visual::WmVisual};

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
        client_id: ClientID,
        config: &WmConfig,
    ) -> Result<(), Box<dyn Error>>;

    fn configure(
        &self,
        connection: &XCBConnection,
        event: &ConfigureRequestEvent,
    ) -> Result<(), Box<dyn Error>>;
}
