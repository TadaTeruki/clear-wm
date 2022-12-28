use zym_model::common::session::CairoImpl;

pub struct WmCairo<'a> {
    cairo_connection: &'a cairo::XCBConnection,
    cairo_visual_type: &'a cairo::XCBVisualType,
}

impl<'a> WmCairo<'a> {
    pub fn new(
        cairo_connection_: &'a cairo::XCBConnection,
        cairo_visual_type_: &'a cairo::XCBVisualType,
    ) -> Self {
        Self {
            cairo_connection: cairo_connection_,
            cairo_visual_type: cairo_visual_type_,
        }
    }
}
impl<'a> CairoImpl<'a> for WmCairo<'a> {
    fn cairo_connection(&self) -> &'a cairo::XCBConnection {
        self.cairo_connection
    }

    fn cairo_visual_type(&self) -> &'a cairo::XCBVisualType {
        self.cairo_visual_type
    }
}
