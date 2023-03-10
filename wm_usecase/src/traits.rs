use std::error::Error;

use x11rb::protocol::xproto::Window;

pub trait ClientUseCaseImpl<'a> {
    fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>>;

    fn start_to_drag_client(
        &mut self,
        window: Window,
        relative_x: i32,
        relative_y: i32,
    ) -> Result<(), Box<dyn Error>>;

    fn draw_client(&mut self, window: Window) -> Result<(), Box<dyn Error>>;

    fn drag_client(&mut self, cursor_x: i32, cursor_y: i32) -> Result<(), Box<dyn Error>>;

    fn release_dragging_client(&mut self) -> Result<(), Box<dyn Error>>;

    fn remove_client(&mut self, window: Window) -> Result<(), Box<dyn Error>>;

    fn activate_client(&self, window: Window) -> Result<(), Box<dyn Error>>;
}
