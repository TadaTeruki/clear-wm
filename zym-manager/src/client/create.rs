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
    client::{geometry::ClientGeometry, ClientID, WindowType, WmClient},
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
        let app_geom = {
            let g = connection.get_geometry(window)?.reply()?;
            ClientGeometry::from_app_absolute(g.x, g.y, g.width, g.height, config)
        };

        let (frame, frame_surface) =
            { create_frame(connection, screen, visual, config, &app_geom)? };

        let mut client_id = self.last_client_id;

        while self.client_container.contains_key(&client_id) {
            client_id += 1;
        }

        self.client_index
            .insert(window, (client_id, WindowType::App));
        self.client_index
            .insert(frame, (client_id, WindowType::Frame));

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
    config: &WmConfig,
    geom: &ClientGeometry,
) -> Result<(Window, XCBSurface), Box<dyn Error>> {
    let win = connection.generate_id()?;
    let colormap = connection.generate_id()?;

    let frame_geom = geom.to_frame(config);

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
        frame_geom.x,
        frame_geom.y,
        frame_geom.width,
        frame_geom.height,
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
        frame_geom.width as i32,
        frame_geom.height as i32,
    )?;

    Ok((win, sfc))
}
