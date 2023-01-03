use std::collections::HashMap;

use wm_config::WmConfig;
use wm_model::entity::{
    client::{ClientID, WindowType},
    visual::WmVisual,
};
use x11rb::{
    protocol::xproto::{Screen, Window},
    xcb_ffi::XCBConnection,
};

use super::client::WmClient;

pub struct WmClientManager<'a> {
    pub(crate) connection: &'a XCBConnection,
    pub(crate) screen: &'a Screen,
    pub(crate) visual: &'a WmVisual,
    pub(crate) config: &'a WmConfig,

    pub(crate) last_client_id: ClientID,
    pub(crate) client_index: HashMap<Window, (ClientID, WindowType)>,
    pub(crate) client_container: HashMap<ClientID, WmClient>,
}

impl<'a> WmClientManager<'a> {
    pub fn new(
        connection_: &'a XCBConnection,
        screen_: &'a Screen,
        visual_: &'a WmVisual,
        config_: &'a WmConfig,
    ) -> Self {
        Self {
            connection: connection_,
            screen: screen_,
            visual: visual_,
            config: config_,
            last_client_id: 0,
            client_index: HashMap::new(),
            client_container: HashMap::new(),
        }
    }
}
