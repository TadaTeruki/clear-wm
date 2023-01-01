use super::client::ClientID;

#[derive(Debug, Clone, Copy)]
pub struct WmCursorDragInfo {
    pub client_id: ClientID,
    pub relative_x: i32,
    pub relative_y: i32,
}
