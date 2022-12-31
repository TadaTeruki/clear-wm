use zym_config::WmConfig;
use zym_model::entity::geometry::Geometry;

pub struct ClientGeometry {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl ClientGeometry {
    pub fn from_app_absolute(geom: Geometry, config: &WmConfig) -> Self {
        let titlebar_height = config.client.frame.titlebar_height;

        Self {
            x: geom.x,
            y: geom.y - titlebar_height as i16,
            width: geom.width,
            height: geom.height + titlebar_height as u16,
        }
    }

    pub fn from_frame(geom: Geometry, config: &WmConfig) -> Self {
        let border_width = config.client.frame.border_width;

        Self {
            x: geom.x + border_width as i16,
            y: geom.y + border_width as i16,
            width: geom.width - border_width as u16 * 2,
            height: geom.height - border_width as u16 * 2,
        }
    }

    pub fn to_frame(&self, config: &WmConfig) -> Geometry {
        let border_width = config.client.frame.border_width;

        Geometry {
            x: self.x - border_width as i16,
            y: self.y - border_width as i16,
            width: self.width + border_width as u16 * 2,
            height: self.height + border_width as u16 * 2,
        }
    }

    pub fn to_app_relative(&self, config: &WmConfig) -> Geometry {
        let border_width = config.client.frame.border_width;
        let titlebar_height = config.client.frame.titlebar_height;

        Geometry {
            x: border_width as i16,
            y: border_width as i16 + titlebar_height as i16,
            width: self.width,
            height: self.height - titlebar_height as u16,
        }
    }
}

pub fn app_relative_position(config: &WmConfig) -> (i16, i16) {
    let border_width = config.client.frame.border_width;
    let titlebar_height = config.client.frame.titlebar_height;
    (
        border_width as i16,
        border_width as i16 + titlebar_height as i16,
    )
}
