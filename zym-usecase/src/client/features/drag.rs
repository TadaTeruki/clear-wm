use std::error::Error;

use x11rb::protocol::xproto::Window;
use zym_model::entity::client::WindowType;

use crate::client::usecase::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn start_to_drag_client(
        &mut self,
        window: Window,
        relative_x: i32,
        relative_y: i32,
    ) -> Result<(), Box<dyn Error>> {
        if let Some((client_id, WindowType::Frame)) = self.client_manager.query(window) {
            self.cursor_manager
                .start_to_drag_client(client_id, relative_x, relative_y);
        }
        Ok(())
    }

    pub fn drag_client(&mut self, cursor_x: i32, cursor_y: i32) -> Result<(), Box<dyn Error>> {
        if let Some(drag_info) = self.cursor_manager.get_drag_info() {
            let (client_id, relative_x, relative_y) = (
                drag_info.client_id,
                drag_info.relative_x,
                drag_info.relative_y,
            );
            self.client_manager.move_to(
                client_id,
                cursor_x - relative_x,
                cursor_y - relative_y,
                WindowType::Frame,
            )?;
        }
        Ok(())
    }

    pub fn release_dragging_client(&mut self) -> Result<(), Box<dyn Error>> {
        self.cursor_manager.release_dragging_client();
        Ok(())
    }
}
