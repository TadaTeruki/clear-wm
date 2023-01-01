use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct WindowSystemGeometry {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl Geometry {
    pub fn new(x_: i32, y_: i32, width_: i32, height_: i32) -> Self {
        Self {
            x: x_,
            y: y_,
            width: width_,
            height: height_,
        }
    }

    pub fn for_system_api(&self) -> Result<WindowSystemGeometry, Box<dyn Error>> {
        let mut alt = *self;

        if alt.width < 0 {
            alt.width = 0
        }
        if alt.height < 0 {
            alt.height = 0
        }

        if alt.width > u16::MAX as i32 {
            alt.width = u16::MAX as i32
        }
        if alt.height > u16::MAX as i32 {
            alt.height = u16::MAX as i32
        }

        Ok(WindowSystemGeometry {
            x: alt.x as i16,
            y: alt.y as i16,
            width: alt.width.try_into()?,
            height: alt.height.try_into()?,
        })
    }
}
