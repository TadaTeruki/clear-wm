use std::error::Error;

use cairo::XCBSurface;
use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ChangeWindowAttributesAux, ColormapAlloc, ConnectionExt, CreateWindowAux, EventMask,
        Window, WindowClass,
    },
};
use zym_model::entity::{
    client::{ClientID, WindowType, WmClient},
    geometry::Geometry,
};

use crate::client::{
    geometry::{app_relative_position, ClientGeometry},
    manager::WmClientManager,
};

impl<'a> WmClientManager<'a> {
    fn create_frame(
        &mut self,
        geom: &ClientGeometry,
    ) -> Result<(Window, XCBSurface), Box<dyn Error>> {
        let win = self.connection.generate_id()?;

        let colormap = self.connection.generate_id()?;

        self.connection.create_colormap(
            ColormapAlloc::NONE,
            colormap,
            self.screen.root,
            self.visual.visual_type.visual_id,
        )?;

        let win_aux = CreateWindowAux::new()
            .event_mask(
                EventMask::EXPOSURE
                    | EventMask::SUBSTRUCTURE_NOTIFY
                    | EventMask::BUTTON_PRESS
                    | EventMask::BUTTON_RELEASE
                    | EventMask::POINTER_MOTION
                    | EventMask::ENTER_WINDOW,
            )
            .background_pixel(0)
            .border_pixel(0)
            .colormap(colormap);

        let frame_geom = geom.to_frame(self.config).for_system_api()?;

        self.connection.create_window(
            self.visual.depth,
            win,
            self.screen.root,
            frame_geom.x,
            frame_geom.y,
            frame_geom.width,
            frame_geom.height,
            0,
            WindowClass::INPUT_OUTPUT,
            self.visual.visual_type.visual_id,
            &win_aux,
        )?;

        let cairo_connection = unsafe {
            cairo::XCBConnection::from_raw_none(self.connection.get_raw_xcb_connection() as _)
        };

        let mut v = self.visual.visual_type;
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

    pub fn create_client(&mut self, window: Window) -> Result<ClientID, Box<dyn Error>> {
        let app_geom = {
            let g = self.connection.get_geometry(window)?.reply()?;
            ClientGeometry::from_app_absolute(
                Geometry {
                    x: g.x.into(),
                    y: g.y.into(),
                    width: g.width.into(),
                    height: g.height.into(),
                },
                self.config,
            )
            .fix_position()
        };

        let (frame, frame_surface) = { self.create_frame(&app_geom)? };

        let (rx, ry) = app_relative_position(self.config);

        self.connection
            .reparent_window(window, frame, rx as i16, ry as i16)?;

        self.connection.change_window_attributes(
            window,
            &ChangeWindowAttributesAux::new().event_mask(EventMask::BUTTON_PRESS),
        )?;

        let mut client_id = self.last_client_id;

        while self.client_container.contains_key(&client_id) {
            client_id += 1;
        }

        self.client_index
            .insert(window, (client_id, WindowType::ComposedApp));
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
