use std::error::Error;

use x11rb::protocol::xproto::Window;
use zym_model::entity::geometry::Geometry;

use super::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn configure_window(
        &self,
        window: Window,
        x_: i16,
        y_: i16,
        width_: u16,
        height_: u16,
    ) -> Result<(), Box<dyn Error>> {
        self.client_manager.configure_window(
            window,
            Geometry {
                x: x_,
                y: y_,
                width: width_,
                height: height_,
            },
        )
    }
}
