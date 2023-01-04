use std::error::Error;

pub trait ClientDrawingDeviceImpl {
    fn draw(&self, client_title: String) -> Result<(), Box<dyn Error>>;
}
