use std::error::Error;

use crate::common::ClientSessionImpl;
use x11rb::protocol::xproto::{ConfigureRequestEvent, Window};

use super::WmSession;

impl<'a> ClientSessionImpl<'a> for WmSession<'a> {
    fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        let client_id = self.manager.create(
            self.connection,
            self.screen,
            self.visual,
            self.config,
            window,
        )?;
        self.manager.map(self.connection, self.config, client_id)?;
        Ok(())
    }

    fn configure_window(&self, event: &ConfigureRequestEvent) -> Result<(), Box<dyn Error>> {
        self.manager.configure_window(
            self.connection,
            event.window,
            event.x,
            event.y,
            event.width,
            event.height,
        )
    }
}
