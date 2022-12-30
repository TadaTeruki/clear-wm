use std::error::Error;

use cairo::XCBSurface;
use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ColormapAlloc, ConnectionExt, CreateWindowAux, EventMask, Screen, Window, WindowClass,
    },
    xcb_ffi::XCBConnection,
};
use zym_config::WmConfig;
use zym_model::entity::{
    client::{ClientID, WmClient},
    visual::WmVisual,
};

use crate::manager::WmClientManager;
impl WmClientManager {
    pub fn create_client(
        &mut self,
        connection: &XCBConnection,
        screen: &Screen,
        visual: &WmVisual,
        config: &WmConfig,
        window: Window,
    ) -> Result<ClientID, Box<dyn Error>> {
        let client_config = &config.client;
        let app_geom = connection.get_geometry(window)?.reply()?;

        let (frame, frame_surface) = {
            let border_width = client_config.frame.border_width;
            let titlebar_height = client_config.frame.titlebar_height;
            create_frame(
                connection,
                screen,
                visual,
                app_geom.x - border_width as i16,
                app_geom.y - border_width as i16,
                app_geom.width + border_width * 2,
                app_geom.height + border_width * 2 + titlebar_height,
            )?
        };

        let mut client_id = self.last_client_id;

        while self.client_container.contains_key(&client_id) {
            client_id += 1;
        }

        self.client_index.insert(window, client_id);
        self.client_index.insert(frame, client_id);

        self.client_container.insert(
            client_id,
            WmClient::new(client_id, window, frame, frame_surface),
        );

        self.last_client_id = client_id;

        Ok(client_id)
    }
}

fn create_frame(
    connection: &XCBConnection,
    screen: &Screen,
    visual: &WmVisual,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
) -> Result<(Window, XCBSurface), Box<dyn Error>> {
    let win = connection.generate_id()?;
    let colormap = connection.generate_id()?;

    connection.create_colormap(ColormapAlloc::NONE, colormap, screen.root, visual.visual_id)?;

    let win_aux = CreateWindowAux::new()
        .event_mask(
            EventMask::EXPOSURE
                | EventMask::SUBSTRUCTURE_NOTIFY
                | EventMask::BUTTON_PRESS
                | EventMask::BUTTON_RELEASE
                | EventMask::POINTER_MOTION
                | EventMask::ENTER_WINDOW,
        )
        .background_pixel(screen.white_pixel);

    connection.create_window(
        visual.depth,
        win,
        screen.root,
        x,
        y,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        visual.visual_id,
        &win_aux,
    )?;

    let cairo_connection =
        unsafe { cairo::XCBConnection::from_raw_none(connection.get_raw_xcb_connection() as _) };

    let mut v = visual.visual_type;
    let visual_type = unsafe { cairo::XCBVisualType::from_raw_none(&mut v as *mut _ as _) };

    let sfc = cairo::XCBSurface::create(
        &cairo_connection,
        &cairo::XCBDrawable(win),
        &visual_type,
        width as i32,
        height as i32,
    )?;

    Ok((win, sfc))
}
