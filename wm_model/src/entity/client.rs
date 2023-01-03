use x11rb::protocol::xproto::Window;

#[derive(Debug, Clone, Copy)]
pub enum WindowType {
    ComposedApp,
    UncomposedApp,
    Frame,
}

pub struct WmClient {
    pub app: Window,
    pub frame: Window,
    pub frame_surface: cairo::XCBSurface,
}

impl WmClient {
    pub fn new(app_: Window, frame_: Window, frame_surface_: cairo::XCBSurface) -> Self {
        Self {
            app: app_,
            frame: frame_,
            frame_surface: frame_surface_,
        }
    }
}
