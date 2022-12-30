#[derive(Debug, Clone, Copy)]
pub struct Geometry {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl Geometry {
    pub fn new(x_: i16, y_: i16, width_: u16, height_: u16) -> Self {
        Self {
            x: x_,
            y: y_,
            width: width_,
            height: height_,
        }
    }
}
