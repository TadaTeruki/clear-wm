use std::error::Error;

pub trait ClientDrawingDeviceImpl {
    fn draw(&self) -> Result<(), Box<dyn Error>>;
}
