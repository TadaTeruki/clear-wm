use std::error::Error;

use x11rb::protocol::xproto::Window;

use crate::entity::client::WmClient;

pub trait CollectionManagerImpl {
    fn push(&mut self, client: WmClient) -> Result<Option<&WmClient>, Box<dyn Error>>;

    fn query_from_window(&self, window: Window) -> Result<Option<&WmClient>, Box<dyn Error>>;

    fn remove(&mut self, window: Window) -> Result<(), Box<dyn Error>>;
}
