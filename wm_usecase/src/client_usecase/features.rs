pub mod control;
pub mod drag;
pub mod draw;
pub mod production;

use std::error::Error;

use x11rb::protocol::xproto::Window;

use crate::traits::ClientUseCaseImpl;

use super::usecase::WmClientUseCase;

impl<'a> ClientUseCaseImpl<'a> for WmClientUseCase<'a> {
    fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        self.usecase_compose_client(window)
    }

    fn start_to_drag_client(
        &mut self,
        window: Window,
        relative_x: i32,
        relative_y: i32,
    ) -> Result<(), Box<dyn Error>> {
        self.usecase_start_to_drag_client(window, relative_x, relative_y)
    }

    fn draw_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        self.usecase_draw_client(window)
    }

    fn drag_client(&mut self, cursor_x: i32, cursor_y: i32) -> Result<(), Box<dyn Error>> {
        self.usecase_drag_client(cursor_x, cursor_y)
    }

    fn release_dragging_client(&mut self) -> Result<(), Box<dyn Error>> {
        self.usecase_release_dragging_client()
    }

    fn remove_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        self.usecase_remove_client(window)
    }

    fn activate_client(&self, window: Window) -> Result<(), Box<dyn Error>> {
        self.usecase_activate_client(window)
    }
}
