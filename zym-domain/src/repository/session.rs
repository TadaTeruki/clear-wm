use x11rb::{connection::Connection, protocol::xproto::Screen};

pub trait SessionRepository<'a, C: Connection> {
    fn connection(&self) -> &'a C;
    fn screen(&self) -> &'a Screen;
}
