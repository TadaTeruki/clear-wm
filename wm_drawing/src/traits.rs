use std::error::Error;

pub trait ClientDrawDeviceImpl {
    fn draw(&self) -> Result<(), Box<dyn Error>>;
}
