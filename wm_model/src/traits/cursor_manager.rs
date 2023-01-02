use crate::entity::{client::ClientID, cursor::WmCursorDragInfo, geometry::Geometry};

pub trait CursorManagerImpl {
    fn start_to_drag_client(
        &mut self,
        client_id_: ClientID,
        relative_x_: i32,
        relative_y_: i32,
        first_frame_geom: Geometry,
    );

    fn get_drag_info(&self) -> Option<WmCursorDragInfo>;

    fn release_dragging_client(&mut self);
}
