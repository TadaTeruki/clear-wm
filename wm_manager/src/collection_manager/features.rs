use std::error::Error;

use wm_model::{entity::client::WmClient, traits::collection_manager::CollectionManagerImpl};
use x11rb::protocol::xproto::Window;

use super::types::WmCollectionManager;

impl CollectionManagerImpl for WmCollectionManager {
    fn push(&mut self, client: WmClient) -> Result<Option<&WmClient>, Box<dyn Error>> {
        let mut client_id = self.last_client_id;

        while self.client_container.contains_key(&client_id) {
            client_id += 1;
        }

        self.client_index.insert(client.app, client_id);
        self.client_index.insert(client.frame, client_id);

        self.client_container.insert(
            client_id,
            WmClient::new(client.app, client.frame, client.frame_surface),
        );

        self.last_client_id = client_id;

        Ok(self.client_container.get(&client_id))
    }

    fn query_from_window(&self, window: Window) -> Result<Option<&WmClient>, Box<dyn Error>> {
        if let Some(client_id) = self.client_index.get(&window) {
            if let Some(client) = self.client_container.get(client_id) {
                return Ok(Some(client));
            }
        }
        Ok(None)
    }

    fn remove(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        let (app, frame, client_id) = {
            if let Some(client_id_) = self.client_index.get(&window) {
                if let Some(client_) = self.client_container.get(client_id_) {
                    (client_.app, client_.frame, client_id_)
                } else {
                    return Ok(());
                }
            } else {
                return Ok(());
            }
        };

        self.client_container.remove(client_id);
        self.client_index.remove(&app);
        self.client_index.remove(&frame);

        Ok(())
    }
}
