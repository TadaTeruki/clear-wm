use std::error::Error;

use x11rb::protocol::xproto::Window;

use super::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        let client_id = self.client_manager.create(window)?;
        self.client_manager.map(client_id)?;
        /*
        let (move_x, move_y): (i16, i16) = (0,0);
        if let Some(frame_geom) = self
            .client_manager
            .get_geometry(client_id, WindowType::Frame)?
        {

            if frame_geom.x+self.
        }*/

        //self.client_manager.move_resize(client_id, geom, window_type);
        Ok(())
    }
}
