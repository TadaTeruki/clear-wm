use std::error::Error;

use log::warn;
use wm_model::entity::{
    client::{WindowType, WmClient},
    geometry::Geometry,
};
use x11rb::protocol::xproto::ConnectionExt;

use crate::client_manager::types::manager::WmClientManager;

impl<'a> WmClientManager<'a> {
    pub fn get_client_geometry(
        &self,
        client: &WmClient,
        window_type: WindowType,
    ) -> Result<Option<Geometry>, Box<dyn Error>> {
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
