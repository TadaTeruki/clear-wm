use x11rb::protocol::xproto::Window;

pub type ClientID = u32;
pub struct WmClient {
    pub client_id: ClientID,
    pub window: Window,
    pub frame: Window,
    pub frame_surface: cairo::XCBSurface,
}

impl WmClient {
    pub fn new(
        client_id_: ClientID,
        window_: Window,
        frame_: Window,
        frame_surface_: cairo::XCBSurface,
    ) -> Self {
        Self {
            client_id: client_id_,
            window: window_,
            frame: frame_,
            frame_surface: frame_surface_,
        }
    }
}
