use zym_model::entity::cursor::WmCursorDragInfo;
use zym_model::{common::manager::CursorManagerImpl, entity::client::ClientID};

pub struct WmCursorManager {
    drag: Option<WmCursorDragInfo>,
}

impl WmCursorManager {
    pub fn new() -> Self {
        Self { drag: None }
    }
}

impl Default for WmCursorManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CursorManagerImpl for WmCursorManager {
    fn start_to_drag_client(&mut self, client_id_: ClientID, relative_x_: i32, relative_y_: i32) {
        self.drag = Some(WmCursorDragInfo {
            client_id: client_id_,
            relative_x: relative_x_,
            relative_y: relative_y_,
        })
    }

    fn get_drag_info(&self) -> Option<WmCursorDragInfo> {
        self.drag
    }

    fn release_dragging_client(&mut self) {
        self.drag = None;
    }
}
