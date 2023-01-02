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
    pub(super) connection: &'a XCBConnection,
    pub(super) screen: &'a Screen,
    pub(super) visual: &'a WmVisual,
    pub(super) config: &'a WmConfig,

    pub(super) last_client_id: ClientID,
    pub(super) client_index: HashMap<Window, (ClientID, WindowType)>,
    pub(super) client_container: HashMap<ClientID, WmClient>,
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
