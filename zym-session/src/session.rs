pub mod client;
pub mod event;

use x11rb::protocol::xproto::Screen;
use x11rb::xcb_ffi::XCBConnection;
use zym_config::WmConfig;

use x11rb::errors::ReplyError;
use zym_model::common::manager::ClientManagerImpl;
use zym_model::entity::visual::WmVisual;

pub struct WmSession<'a> {
    connection: &'a XCBConnection,
    screen: &'a Screen,
    visual: &'a WmVisual,
    config: &'a WmConfig,
    manager: &'a mut dyn ClientManagerImpl<'a>,
}

impl<'a> WmSession<'a> {
    pub fn new(
        connection_: &'a XCBConnection,
        screen_: &'a Screen,
        visual_: &'a WmVisual,
        config_: &'a WmConfig,
        manager_: &'a mut dyn ClientManagerImpl<'a>,
    ) -> Result<Self, ReplyError> {
        Ok(Self {
            connection: connection_,
            screen: screen_,
            visual: visual_,
            config: config_,
            manager: manager_,
        })
    }
}
