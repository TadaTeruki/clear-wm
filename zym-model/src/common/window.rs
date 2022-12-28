use x11rb::protocol::xproto::Window;

use super::session::SessionImpl;
use std::error::Error;

pub trait WindowImpl: Sized {
    fn window(&self) -> Window;
}

pub trait DrawableImpl<'a>: WindowImpl {
    fn create(
        session: &dyn SessionImpl<'a>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<Self, Box<dyn Error>>;

    fn draw(&self) -> Result<(), Box<dyn Error>>;
}
