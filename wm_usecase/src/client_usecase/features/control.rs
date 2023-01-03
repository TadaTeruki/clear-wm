use std::error::Error;

use x11rb::protocol::xproto::Window;

use crate::client_usecase::types::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn usecase_activate_client(&self, window: Window) -> Result<(), Box<dyn Error>> {
        if let Some(client) = self.collection_manager.query_from_window(window)? {
            if let Some(prev_active_client) = self
                .collection_manager
                .query_from_window(self.client_manager.get_focus()?)?
            {
                self.client_manager.grab(prev_active_client)?;
            }

            self.client_manager.set_focus(client)?;
            self.client_manager.raise(client)?;
            self.client_manager.ungrab(client)?;
        }

        Ok(())
    }
}
