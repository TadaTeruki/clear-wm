use x11rb::{
    protocol::xproto::{Screen, Visualid},
    xcb_ffi,
};
use zym_config::WmConfig;

pub trait SessionImpl<'a> {
    fn connection(&self) -> &'a xcb_ffi::XCBConnection;
    fn screen(&self) -> &'a Screen;
    fn config(&self) -> &'a WmConfig;
    fn visual_repository(&self) -> &'a dyn VisualRepositoryImpl;
    fn cairo_repository(&self) -> &'a dyn CairoRepositoryImpl<'a>;
}

pub trait VisualRepositoryImpl {
    fn depth(&self) -> u8;
    fn visual_id(&self) -> Visualid;
}

pub trait CairoRepositoryImpl<'a> {
    fn cairo_connection(&self) -> &'a cairo::XCBConnection;
    fn cairo_visual_type(&self) -> &'a cairo::XCBVisualType;
}
