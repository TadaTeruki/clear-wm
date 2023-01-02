use std::error::Error;

use log::warn;
use wm_model::entity::{
    client::{ClientID, WindowType},
    geometry::Geometry,
};
use x11rb::protocol::xproto::ConnectionExt;

use crate::client_manager::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn get_client_geometry(
        &self,
        client_id: ClientID,
        window_type: WindowType,
    ) -> Result<Option<Geometry>, Box<dyn Error>> {
        let client = {
            if let Some(client_) = self.client_container.get(&client_id) {
                client_
            } else {
                warn!("client not found");
                return Ok(None);
            }
        };
        let geom = match window_type {
            WindowType::Frame => self.connection.get_geometry(client.frame)?.reply()?,
            WindowType::ComposedApp => self.connection.get_geometry(client.app)?.reply()?,
            _ => {
                warn!("unable to get geometry. (window type: {:?})", window_type);
                return Ok(None);
            }
        };

        Ok(Some(Geometry {
            x: geom.x.into(),
            y: geom.y.into(),
            width: geom.width.into(),
            height: geom.height.into(),
        }))
    }
}
