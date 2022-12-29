use std::error::Error;

use x11rb::protocol::{
    xproto::{ConfigureRequestEvent, Window},
    Event,
};

pub trait SessionImpl<'a> {
    fn wait_for_event(&self) -> Result<Event, Box<dyn Error>>;
    fn poll_for_event(&self) -> Result<Option<Event>, Box<dyn Error>>;
    fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>>;
    fn configure_window(&self, event: &ConfigureRequestEvent) -> Result<(), Box<dyn Error>>;
}
