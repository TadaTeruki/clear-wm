use std::error::Error;

use x11rb::protocol::xproto::Window;
use zym_model::common::manager::ClientManagerImpl;

use crate::common::ClientUseCaseImpl;

pub struct WmClientUseCase<'a> {
    pub client_manager: &'a mut dyn ClientManagerImpl<'a>,
}

impl<'a> WmClientUseCase<'a> {
    pub fn new(client_manager_: &'a mut dyn ClientManagerImpl<'a>) -> Self {
        Self {
            client_manager: client_manager_,
        }
    }
}

impl<'a> ClientUseCaseImpl<'a> for WmClientUseCase<'a> {
    fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        self.compose_client(window)
    }
    fn configure_window(
        &self,
        window: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<(), Box<dyn Error>> {
        self.configure_window(window, x, y, width, height)
    }
}
