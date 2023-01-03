use wm_config::WmConfig;

pub struct WmDrawingDevice<'a> {
    pub(super) surface: &'a cairo::XCBSurface,
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) config: &'a WmConfig,
}

impl<'a> WmDrawingDevice<'a> {
    pub fn new(
        surface_: &'a cairo::XCBSurface,
        width_: i32,
        height_: i32,
        config_: &'a WmConfig,
    ) -> Self {
        Self {
            surface: surface_,
            width: width_,
            height: height_,
            config: config_,
        }
    }
}
