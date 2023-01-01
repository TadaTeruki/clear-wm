use std::cmp::max;
use zym_config::WmConfig;
use zym_model::entity::geometry::Geometry;

#[derive(Debug, Clone, Copy)]
pub struct ClientGeometry {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl ClientGeometry {
    pub fn from_app_absolute(geom: Geometry, config: &WmConfig) -> Self {
        let titlebar_height = config.client.frame.titlebar_height;

        Self {
            x: geom.x,
            y: geom.y - titlebar_height as i32,
            width: geom.width,
            height: geom.height + titlebar_height as i32,
        }
    }

    pub fn from_frame(geom: Geometry, config: &WmConfig) -> Self {
        let border_width = config.client.frame.border_width;

        Self {
            x: geom.x + border_width as i32,
            y: geom.y + border_width as i32,
            width: geom.width - border_width as i32 * 2,
            height: geom.height - border_width as i32 * 2,
        }
    }

    pub fn to_frame(&self, config: &WmConfig) -> Geometry {
        let border_width = config.client.frame.border_width;

        Geometry {
            x: self.x - border_width as i32,
            y: self.y - border_width as i32,
            width: self.width + border_width as i32 * 2,
            height: self.height + border_width as i32 * 2,
        }
    }

    pub fn to_app_relative(&self, config: &WmConfig) -> Geometry {
        let border_width = config.client.frame.border_width;
        let titlebar_height = config.client.frame.titlebar_height;

        Geometry {
            x: border_width as i32,
            y: border_width as i32 + titlebar_height as i32,
            width: self.width,
            height: self.height - titlebar_height as i32,
        }
    }

    pub fn fix_position(&self) -> Self {
        Self {
            x: max(self.x, 0),
            y: max(self.y, 0),
            width: self.width,
            height: self.height,
        }
    }
}

pub fn app_relative_position(config: &WmConfig) -> (i32, i32) {
    let border_width = config.client.frame.border_width;
    let titlebar_height = config.client.frame.titlebar_height;
    (
        border_width as i32,
        border_width as i32 + titlebar_height as i32,
    )
}
