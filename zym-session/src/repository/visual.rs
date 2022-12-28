use x11rb::protocol::xproto::Visualid;
use zym_model::common::session::VisualRepositoryImpl;

pub struct WmVisualRepository {
    depth: u8,
    visual_id: Visualid,
}

impl WmVisualRepository {
    pub fn new(depth_: u8, visual_id_: Visualid) -> Self {
        Self {
            depth: depth_,
            visual_id: visual_id_,
        }
    }
}

impl VisualRepositoryImpl for WmVisualRepository {
    fn depth(&self) -> u8 {
        self.depth
    }

    fn visual_id(&self) -> Visualid {
        self.visual_id
    }
}
