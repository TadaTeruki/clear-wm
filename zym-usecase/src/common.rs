use std::error::Error;

use x11rb::protocol::xproto::Window;

pub trait ClientUseCaseImpl<'a> {
    fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>>;

    fn configure_window(
        &self,
        window: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<(), Box<dyn Error>>;
}
