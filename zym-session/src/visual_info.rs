use x11rb::protocol::xproto::Visualid;
use zym_model::common::session::VisualInfoImpl;

pub struct WmVisualInfo {
    depth: u8,
    visual_id: Visualid,
}

impl WmVisualInfo {
    pub fn new(depth_: u8, visual_id_: Visualid) -> Self {
        Self {
            depth: depth_,
            visual_id: visual_id_,
        }
    }
}

impl VisualInfoImpl for WmVisualInfo {
    fn depth(&self) -> u8 {
        self.depth
    }

    fn visual_id(&self) -> Visualid {
        self.visual_id
    }
}
