use std::error::Error;

use pango::{traits::FontMapExt, FontDescription, Layout};
use pangocairo::{show_layout, FontMap};

use crate::{draw_device::types::WmDrawingDevice, traits::ClientDrawingDeviceImpl};

impl<'a> ClientDrawingDeviceImpl for WmDrawingDevice<'a> {
    fn draw(&self, client_title: String) -> Result<(), Box<dyn Error>> {
        let cr = cairo::Context::new(self.surface)?;
        let pango_cr = unsafe { pangocairo::cairo::Context::from_raw_none(cr.to_raw_none() as _) };

        cr.set_source_rgba(0.7, 0.7, 0.7, 0.5);
        cr.paint()?;

        let border_width = self.config.client.frame.border_width;
        let titlebar_height = self.config.client.frame.titlebar_height;

        cr.rectangle(
            border_width.into(),
            border_width.into(),
            (self.width - border_width * 2).into(),
            (self.height - border_width * 2).into(),
        );

        cr.set_source_rgba(0.95, 0.95, 0.95, 0.95);
        cr.fill()?;

        let layout = {
            let font_map = FontMap::default();
            let pango_ctx = font_map.create_context();
            let font_desc = FontDescription::from_string("ubuntu");
            let layout_ = Layout::new(&pango_ctx);

            font_map.load_font(&pango_ctx, &font_desc);
            layout_.set_font_description(Some(&font_desc));
            layout_
        };

        layout.set_width(titlebar_height * pango::SCALE);
        layout.set_height(titlebar_height * pango::SCALE);
        layout.set_text(client_title.as_str());
        cr.move_to(border_width.into(), border_width.into());
        cr.set_source_rgba(0.2, 0.2, 0.2, 1.0);
        show_layout(&pango_cr, &layout);

        self.surface.flush();
        Ok(())
    }
}
