use std::error::Error;

use log::warn;
use zym_draw::{common::ClientDrawDeviceImpl, device::WmDrawDevice};
use zym_model::{
    common::manager::ClientManagerImpl,
    entity::client::{ClientID, WindowType},
};

use crate::client::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn draw_client_frame(&self, client_id: ClientID) -> Result<(), Box<dyn Error>> {
        if let (Some(client), Some(geom)) = (
            self.client_container.get(&client_id),
            self.get_geometry(client_id, WindowType::Frame)?,
        ) {
            let draw_device =
                WmDrawDevice::new(&client.frame_surface, geom.width, geom.height, self.config);
            <WmDrawDevice as ClientDrawDeviceImpl>::draw(&draw_device)?;
        } else {
            warn!("surface not found");
        }
        Ok(())
    }
}
