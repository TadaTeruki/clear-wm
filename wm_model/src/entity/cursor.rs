use x11rb::protocol::xproto::Window;

use super::geometry::Geometry;

#[derive(Debug, Clone, Copy)]
pub enum DragMode {
    Move,
    ResizeTop,
    ResizeBottom,
    ResizeLeft,
    ResizeRight,
    ResizeTL, // top and left
    ResizeTR, // top and right
    ResizeBL, // bottom and left
    ResizeBR, // bottom and right
}

#[derive(Debug, Clone, Copy)]
pub struct WmCursorDragInfo {
    pub app: Window,
    pub relative_x: i32,
    pub relative_y: i32,
    pub first_frame_geom: Geometry,
    pub drag_mode: DragMode,
}
