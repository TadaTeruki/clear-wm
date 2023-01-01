use std::cmp::min;
use zym_config::WmConfig;
use zym_model::entity::cursor::{DragMode, WmCursorDragInfo};
use zym_model::entity::geometry::Geometry;
use zym_model::{common::manager::CursorManagerImpl, entity::client::ClientID};

pub struct WmCursorManager<'a> {
    drag: Option<WmCursorDragInfo>,
    pub config: &'a WmConfig,
}

impl<'a> WmCursorManager<'a> {
    pub fn new(config_: &'a WmConfig) -> Self {
        Self {
            drag: None,
            config: config_,
        }
    }
}

impl<'a> CursorManagerImpl for WmCursorManager<'a> {
    fn start_to_drag_client(
        &mut self,
        client_id_: ClientID,
        relative_x_: i32,
        relative_y_: i32,
        frame_geom: Geometry,
    ) {
        let mut drag_mode_ = DragMode::Move;

        let corner_width = min(
            self.config.client.control.drag_corner_width,
            min(frame_geom.width / 2, frame_geom.height / 2),
        );

        if relative_x_ <= corner_width {
            drag_mode_ = DragMode::ResizeLeft;
        } else if relative_x_ >= frame_geom.width - corner_width {
            drag_mode_ = DragMode::ResizeRight;
        }

        if relative_y_ <= corner_width {
            if let DragMode::ResizeRight = drag_mode_ {
                drag_mode_ = DragMode::ResizeTR;
            } else if let DragMode::ResizeLeft = drag_mode_ {
                drag_mode_ = DragMode::ResizeTL;
            } else {
                drag_mode_ = DragMode::ResizeTop;
            }
        } else if relative_y_ >= frame_geom.height - corner_width {
            if let DragMode::ResizeRight = drag_mode_ {
                drag_mode_ = DragMode::ResizeBR;
            } else if let DragMode::ResizeLeft = drag_mode_ {
                drag_mode_ = DragMode::ResizeBL;
            } else {
                drag_mode_ = DragMode::ResizeBottom;
            }
        }

        self.drag = Some(WmCursorDragInfo {
            client_id: client_id_,
            relative_x: relative_x_,
            relative_y: relative_y_,
            first_frame_geom: frame_geom,
            drag_mode: drag_mode_,
        })
    }

    fn get_drag_info(&self) -> Option<WmCursorDragInfo> {
        self.drag
    }

    fn release_dragging_client(&mut self) {
        self.drag = None;
    }
}
