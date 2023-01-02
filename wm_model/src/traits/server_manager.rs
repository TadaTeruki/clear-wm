use std::error::Error;

pub trait ServerManagerImpl {
    fn grab(&mut self) -> Result<(), Box<dyn Error>>;
    fn ungrab(&mut self) -> Result<(), Box<dyn Error>>;
    fn sync(&mut self) -> Result<(), Box<dyn Error>>;
}
