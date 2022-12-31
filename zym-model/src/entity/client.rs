use x11rb::protocol::xproto::Window;

pub type ClientID = u32;

#[derive(Debug, Clone, Copy)]
pub enum WindowType {
    ComposedApp,
    UncomposedApp,
    Frame,
}

pub struct WmClient {
    pub client_id: ClientID,
    pub app: Window,
    pub frame: Window,
    pub frame_surface: cairo::XCBSurface,
}

impl WmClient {
    pub fn new(
        client_id_: ClientID,
        app_: Window,
        frame_: Window,
        frame_surface_: cairo::XCBSurface,
    ) -> Self {
        Self {
            client_id: client_id_,
            app: app_,
            frame: frame_,
            frame_surface: frame_surface_,
        }
    }
}
