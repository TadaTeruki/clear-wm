use std::error::Error;

use x11rb::protocol::Event;

pub trait EventListenerImpl<'a> {
    fn wait(&self) -> Result<Event, Box<dyn Error>>;
    fn poll(&self) -> Result<Option<Event>, Box<dyn Error>>;
}
