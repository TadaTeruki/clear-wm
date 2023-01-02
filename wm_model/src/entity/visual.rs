use x11rb::{
    protocol::xproto::{Screen, VisualClass},
    xcb_ffi::ReplyError,
};

pub struct WmVisual {
    pub depth: u8,
    pub visual_type: XCBVisualType,
}

impl WmVisual {
    pub fn new(screen: &Screen) -> Result<Self, ReplyError> {
        let depth_ = 32;
        Ok(Self {
            depth: depth_,
            visual_type: query_visual_type(screen, depth_).unwrap(),
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

fn query_visual_type(screen: &Screen, depth_: u8) -> Option<XCBVisualType> {
    for depth in &screen.allowed_depths {
        if depth.depth != depth_ {
            continue;
        }
        for visual_type in &depth.visuals {
            if visual_type.class == VisualClass::TRUE_COLOR {
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
    None
}
