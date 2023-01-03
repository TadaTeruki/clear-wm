use wm_config::WmConfig;
use wm_model::entity::visual::WmVisual;
use x11rb::{protocol::xproto::Screen, xcb_ffi::XCBConnection};

pub struct WmClientManager<'a> {
    pub(crate) connection: &'a XCBConnection,
    pub(crate) screen: &'a Screen,
    pub(crate) visual: &'a WmVisual,
    pub(crate) config: &'a WmConfig,
}

impl<'a> WmClientManager<'a> {
    pub fn new(
        connection_: &'a XCBConnection,
        screen_: &'a Screen,
        visual_: &'a WmVisual,
        config_: &'a WmConfig,
    ) -> Self {
        Self {
            connection: connection_,
            screen: screen_,
            visual: visual_,
            config: config_,
        }
    }
}
