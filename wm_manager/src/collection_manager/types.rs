use std::collections::HashMap;

pub type ClientID = u64;

use wm_model::entity::client::WmClient;
use x11rb::protocol::xproto::Window;

pub struct WmCollectionManager {
    pub(super) last_client_id: ClientID,
    pub(super) client_index: HashMap<Window, ClientID>,
    pub(super) client_container: HashMap<ClientID, WmClient>,
}

impl WmCollectionManager {
    pub fn new() -> Self {
        Self {
            last_client_id: 0,
            client_index: HashMap::new(),
            client_container: HashMap::new(),
        }
    }
}

impl Default for WmCollectionManager {
    fn default() -> Self {
        Self::new()
    }
}
