use std::error::Error;

use x11rb::protocol::xproto::ConfigureRequestEvent;

pub trait EventReflectorImpl<'a> {
    fn reflect_on_configure_request(
        &self,
        event: &ConfigureRequestEvent,
    ) -> Result<(), Box<dyn Error>>;
}
