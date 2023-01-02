use std::error::Error;

use log::warn;
use wm_drawing::{drawing_device::device::WmDrawingDevice, traits::ClientDrawingDeviceImpl};
use wm_model::{
    entity::client::{ClientID, WindowType},
    traits::client_manager::ClientManagerImpl,
};

use crate::client_manager::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn draw_client_frame(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        if let (Some(client), Some(geom)) = (
            self.client_container.get(&client_id),
            self.get_geometry(client_id, WindowType::Frame)?,
        ) {
            let draw_device =
                WmDrawingDevice::new(&client.frame_surface, geom.width, geom.height, self.config);
            <WmDrawingDevice as ClientDrawingDeviceImpl>::draw(&draw_device)?;
        } else {
            warn!("surface not found");
        }
        Ok(())
    }
}
