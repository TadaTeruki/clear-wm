use std::error::Error;
use x11rb::protocol::xproto::ButtonPressEvent;

use crate::event_handler::handler::WmEventHandler;

impl<'a> WmEventHandler<'a> {
    pub fn handle_button_press(&mut self, event: &ButtonPressEvent) -> Result<(), Box<dyn Error>> {
        self.client_usecase.start_to_drag_client(
            event.event,
            event.event_x.into(),
            event.event_y.into(),
        )?;
        self.client_usecase.activate_client(event.event)?;
        Ok(())
    }
}
