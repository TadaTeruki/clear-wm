use std::error::Error;

use wm_draw::{drawing_device::types::WmDrawingDevice, traits::ClientDrawingDeviceImpl};
use wm_model::entity::client::WmClient;
use x11rb::protocol::xproto::ConnectionExt;

use crate::client_manager::types::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn draw_client_frame(
        &self,
        client: &WmClient,
        client_title: String,
    ) -> Result<(), Box<dyn Error>> {
        let geom = self.connection.get_geometry(client.frame)?.reply()?;
        let draw_device = WmDrawingDevice::new(
            &client.frame_surface,
            geom.width as i32,
            geom.height as i32,
            self.config,
        );
        <WmDrawingDevice as ClientDrawingDeviceImpl>::draw(&draw_device, client_title)?;
        Ok(())
    }
}
