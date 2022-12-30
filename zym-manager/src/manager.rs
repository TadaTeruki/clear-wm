use std::{collections::HashMap, error::Error};

use x11rb::{
    protocol::xproto::{ConfigureRequestEvent, ConfigureWindowAux, ConnectionExt, Screen, Window},
    xcb_ffi::XCBConnection,
};
use zym_config::WmConfig;
use zym_model::{
    common::manager::ClientManagerImpl,
    entity::{
        client::{ClientID, WmClient},
        visual::WmVisual,
    },
};

pub struct WmClientManager {
    pub last_client_id: ClientID,
    pub client_index: HashMap<Window, ClientID>,
    pub client_container: HashMap<ClientID, WmClient>,
}

impl WmClientManager {
    pub fn new() -> Self {
        Self {
            last_client_id: 0,
            client_index: HashMap::new(),
            client_container: HashMap::new(),
        }
    }
}

impl Default for WmClientManager {
    fn default() -> Self {
        Self::new()
    }
}

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
        client_id: ClientID,
        config: &WmConfig,
    ) -> Result<(), Box<dyn Error>> {
        self.map_client(connection, client_id, config)
    }

    fn configure(
        &self,
        connection: &XCBConnection,
        event: &ConfigureRequestEvent,
    ) -> Result<(), Box<dyn Error>> {
        let aux = ConfigureWindowAux::from_configure_request(event)
            .sibling(None)
            .stack_mode(None);
        connection.configure_window(event.window, &aux)?;
        Ok(())
    }
}
