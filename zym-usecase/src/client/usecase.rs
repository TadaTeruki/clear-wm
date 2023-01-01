use std::error::Error;

use x11rb::protocol::xproto::Window;
use zym_model::common::manager::{ClientManagerImpl, CursorManagerImpl};

use crate::common::ClientUseCaseImpl;

pub struct WmClientUseCase<'a> {
    pub client_manager: &'a mut dyn ClientManagerImpl<'a>,
    pub cursor_manager: &'a mut dyn CursorManagerImpl,
}

impl<'a> WmClientUseCase<'a> {
    pub fn new(
        client_manager_: &'a mut dyn ClientManagerImpl<'a>,
        cursor_manager_: &'a mut dyn CursorManagerImpl,
    ) -> Self {
        Self {
            client_manager: client_manager_,
            cursor_manager: cursor_manager_,
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
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), Box<dyn Error>> {
        self.configure_window(window, x, y, width, height)
    }

    fn start_to_drag_client(
        &mut self,
        window: Window,
        relative_x: i32,
        relative_y: i32,
    ) -> Result<(), Box<dyn Error>> {
        self.start_to_drag_client(window, relative_x, relative_y)
    }

    fn drag_client(&mut self, cursor_x: i32, cursor_y: i32) -> Result<(), Box<dyn Error>> {
        self.drag_client(cursor_x, cursor_y)
    }

    fn release_dragging_client(&mut self) -> Result<(), Box<dyn Error>> {
        self.release_dragging_client()
    }
}
