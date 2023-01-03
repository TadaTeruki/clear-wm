use std::error::Error;

use log::{error, info};
use x11rb::protocol::Event;

use super::types::WmEventHandler;

mod button_press;
mod button_release;
mod client_message;
mod configure_request;
mod enter_notify;
mod expose;
mod map_request;
mod motion_notify;
mod unmap_notify;

impl<'a> WmEventHandler<'a> {
    pub fn handle_event(&mut self) -> Result<(), Box<dyn Error>> {
        let event = self.event_listener.wait()?;
        let mut event_option = Some(event);

        while let Some(event) = event_option {
            info!("X event - {:?}", event);

            match event {
                Event::ButtonPress(ev) => self.handle_button_press(&ev)?,
                Event::ButtonRelease(ev) => self.handle_button_release(&ev)?,
                Event::ClientMessage(ev) => self.handle_client_message(&ev)?,
                Event::ConfigureRequest(ev) => self.handle_configure_request(&ev)?,
                Event::EnterNotify(ev) => self.handle_enter_notify(&ev)?,
                Event::Expose(ev) => self.handle_expose(&ev)?,
                Event::MapRequest(ev) => self.handle_map_request(&ev)?,
                Event::MotionNotify(ev) => self.handle_motion_notify(&ev)?,
                Event::UnmapNotify(ev) => self.handle_unmap_notify(&ev)?,
                Event::Error(err) => error!("{:?}", err),
                _ => {}
            }
            event_option = self.event_listener.poll()?;
        }

        Ok(())
    }
}
