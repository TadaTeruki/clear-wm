use std::error::Error;

use x11rb::protocol::xproto::Window;
use zym_model::entity::geometry::Geometry;

use crate::client::usecase::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn configure_window(
        &self,
        window: Window,
        x_: i32,
        y_: i32,
        width_: i32,
        height_: i32,
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
