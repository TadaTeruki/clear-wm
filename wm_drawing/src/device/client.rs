use std::error::Error;

use crate::traits::ClientDrawDeviceImpl;

use super::WmDrawDevice;

impl<'a> ClientDrawDeviceImpl for WmDrawDevice<'a> {
    fn draw(&self) -> Result<(), Box<dyn Error>> {
        let cr = cairo::Context::new(self.surface)?;

        cr.set_source_rgba(0.7, 0.7, 0.7, 0.5);
        cr.paint()?;

        let border_width = self.config.client.frame.border_width;

        cr.rectangle(
            border_width.into(),
            border_width.into(),
            (self.width - border_width * 2).into(),
            (self.height - border_width * 2).into(),
        );

        cr.set_source_rgba(0.95, 0.95, 0.95, 0.95);
        cr.fill()?;

        self.surface.flush();
        Ok(())
    }
}
