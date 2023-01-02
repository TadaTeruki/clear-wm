pub mod client;

use wm_config::WmConfig;

pub struct WmDrawDevice<'a> {
    surface: &'a cairo::XCBSurface,
    width: i32,
    height: i32,
    config: &'a WmConfig,
}

impl<'a> WmDrawDevice<'a> {
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
