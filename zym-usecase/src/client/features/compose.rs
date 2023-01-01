use std::error::Error;

use x11rb::protocol::xproto::Window;

use crate::client::usecase::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        let client_id = self.client_manager.create(window)?;
        self.client_manager.map(client_id)?;
        Ok(())
    }
}
