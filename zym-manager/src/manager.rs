use std::collections::HashMap;

use x11rb::protocol::xproto::Window;
use zym_model::entity::client::{ClientID, WindowType, WmClient};

pub struct WmClientManager {
    pub last_client_id: ClientID,
    pub client_index: HashMap<Window, (ClientID, WindowType)>,
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
