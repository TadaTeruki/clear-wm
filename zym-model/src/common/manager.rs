use std::error::Error;

use x11rb::protocol::xproto::Window;

use crate::entity::{
    client::{ClientID, WindowType},
    cursor::WmCursorDragInfo,
    geometry::Geometry,
};

pub trait ClientManagerImpl<'a> {
    fn create(&mut self, window: Window) -> Result<ClientID, Box<dyn Error>>;

    fn map(&self, client_id: ClientID) -> Result<(), Box<dyn Error>>;

    fn move_resize(
        &self,
        client_id: ClientID,
        geom: Geometry,
        window_type: WindowType,
    ) -> Result<(), Box<dyn Error>>;

    fn get_geometry(
        &self,
        client_id: ClientID,
        window_type: WindowType,
    ) -> Result<Option<Geometry>, Box<dyn Error>>;

    fn move_to(
        &self,
        client_id: ClientID,
        x: i32,
        y: i32,
        window_type: WindowType,
    ) -> Result<(), Box<dyn Error>>;

    fn resize(
        &self,
        client_id: ClientID,
        width: i32,
        height: i32,
        window_type: WindowType,
    ) -> Result<(), Box<dyn Error>>;

    fn configure_window(&self, window: Window, geom: Geometry) -> Result<(), Box<dyn Error>>;

    fn query_id(&self, window: Window) -> Option<(ClientID, WindowType)>;

    fn draw_frame(&self, client_id: ClientID) -> Result<(), Box<dyn Error>>;

    fn remove(&mut self, client_id: ClientID) -> Result<(), Box<dyn Error>>;

    fn set_focus(&self, client_id: ClientID) -> Result<(), Box<dyn Error>>;

    fn get_focus(&self) -> Result<Option<ClientID>, Box<dyn Error>>;

    fn raise(&self, client_id: ClientID) -> Result<(), Box<dyn Error>>;

    fn grab(&self, client_id: ClientID) -> Result<(), Box<dyn Error>>;

    fn ungrab(&self, client_id: ClientID) -> Result<(), Box<dyn Error>>;
}

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

pub trait ServerManagerImpl {
    fn grab(&mut self) -> Result<(), Box<dyn Error>>;
    fn ungrab(&mut self) -> Result<(), Box<dyn Error>>;
    fn sync(&mut self) -> Result<(), Box<dyn Error>>;
}
