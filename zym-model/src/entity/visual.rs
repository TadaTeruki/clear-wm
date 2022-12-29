use x11rb::{
    connection::Connection,
    protocol::xproto::{Screen, Visualid},
    xcb_ffi::{ReplyError, XCBConnection},
};

pub struct WmVisual {
    pub depth: u8,
    pub visual_id: Visualid,
    pub visual_type: XCBVisualType,
}

impl WmVisual {
    pub fn new(connection: &XCBConnection, screen: &Screen) -> Result<Self, ReplyError> {
        let (depth_, visual_id_) = (screen.root_depth, screen.root_visual);
        Ok(Self {
            depth: depth_,
            visual_id: visual_id_,
            visual_type: query_visual_type(connection, visual_id_).unwrap(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XCBVisualType {
    pub visual_id: u32,
    pub class: u8,
    pub bits_per_rgb_visual_type: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad0: [u8; 4],
}

fn query_visual_type(connection: &XCBConnection, visual_id: u32) -> Option<XCBVisualType> {
    for root in &connection.setup().roots {
        for depth in &root.allowed_depths {
            for visual_type in &depth.visuals {
                if visual_type.visual_id == visual_id {
                    return Some(XCBVisualType {
                        visual_id: visual_type.visual_id,
                        class: visual_type.class.into(),
                        bits_per_rgb_visual_type: visual_type.bits_per_rgb_value,
                        colormap_entries: visual_type.colormap_entries,
                        red_mask: visual_type.red_mask,
                        green_mask: visual_type.green_mask,
                        blue_mask: visual_type.blue_mask,
                        pad0: [0; 4],
                    });
                }
            }
        }
    }
    None
}
