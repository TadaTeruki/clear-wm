use std::{collections::HashMap, error::Error};

use cairo::XCBSurface;
use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ColormapAlloc, ConfigureRequestEvent, ConfigureWindowAux, ConnectionExt, CreateWindowAux,
        EventMask, Screen, SetMode, Window, WindowClass,
    },
    xcb_ffi::XCBConnection,
};
use zym_config::WmConfig;
use zym_model::{
    common::manager::ClientManagerImpl,
    entity::{
        client::{ClientID, WmClient},
        visual::WmVisual,
    },
};

pub struct WmClientManager {
    last_client_id: ClientID,
    client_index: HashMap<Window, ClientID>,
    client_container: HashMap<ClientID, WmClient>,
}

impl WmClientManager {
    pub fn new() -> Self {
        Self {
            last_client_id: 0,
            client_index: HashMap::new(),
            client_container: HashMap::new(),
        }
    }
}

impl Default for WmClientManager {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ClientManagerImpl<'a> for WmClientManager {
    fn create(
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

    fn map(
        &self,
        connection: &XCBConnection,
        client_id: ClientID,
        config: &WmConfig,
    ) -> Result<(), Box<dyn Error>> {
        let client_option = self.client_container.get(&client_id);
        let client_config = &config.client;

        if let Some(client) = client_option {
            let border_width = client_config.frame.border_width as i16;
            let titlebar_height = client_config.frame.titlebar_height as i16;
            connection.grab_server()?;
            connection.change_save_set(SetMode::INSERT, client.window)?;
            connection.reparent_window(
                client.window,
                client.frame,
                border_width,
                border_width + titlebar_height,
            )?;
            connection.map_window(client.frame)?;
            connection.map_window(client.window)?;
            connection.ungrab_server()?;
        }

        Ok(())
    }

    fn configure(
        &self,
        connection: &XCBConnection,
        event: &ConfigureRequestEvent,
    ) -> Result<(), Box<dyn Error>> {
        let aux = ConfigureWindowAux::from_configure_request(event)
            .sibling(None)
            .stack_mode(None);
        connection.configure_window(event.window, &aux)?;
        Ok(())
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
