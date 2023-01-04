use std::error::Error;

use crate::entity::client::WmClient;

pub trait PropertyManagerImpl {
    fn client_title(&self, client: &WmClient) -> Result<String, Box<dyn Error>>;
}
