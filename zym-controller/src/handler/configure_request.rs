use std::error::Error;
use x11rb::protocol::xproto::ConfigureRequestEvent;

use super::WmHandler;

impl<'a> WmHandler<'a> {
    pub fn handle_configure_request(
        &self,
        event: &ConfigureRequestEvent,
    ) -> Result<(), Box<dyn Error>> {
        self.client_usecase.configure_window(
            event.window,
            event.x.into(),
            event.y.into(),
            event.width.into(),
            event.height.into(),
        )?;
        Ok(())
    }
}
