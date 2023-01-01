use zym_model::{
    common::manager::{ClientManagerImpl, CursorManagerImpl},
    entity::client::ClientID,
};

struct WmClientDragInfo {
    client_id: ClientID,
    point_x: i16,
    point_y: i16,
}

pub struct WmCursorManager {
    dragging: Option<WmClientDragInfo>,
}

impl WmCursorManager {
    pub fn new() -> Self {
        Self { dragging: None }
    }
}

impl CursorManagerImpl for WmCursorManager {
    fn start_to_drag_client(&mut self, client_id: ClientID) {}

    fn get_dragging_client(&self) -> Option<ClientID> {
        None
    }

    fn release_dragging_client(&mut self) {}
}
