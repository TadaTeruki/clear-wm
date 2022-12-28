use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ColormapAlloc, ConnectionExt, CreateWindowAux, EventMask, Window, WindowClass,
    },
};

use crate::common::{
    session::SessionImpl,
    window::{DrawableImpl, WindowImpl},
};

pub struct WmClientFrameWindow {
    window: Window,
    surface: cairo::XCBSurface,
}

impl WindowImpl for WmClientFrameWindow {
    fn window(&self) -> Window {
        self.window
    }
}

impl<'a> DrawableImpl<'a> for WmClientFrameWindow {
    fn create(
        session: &dyn SessionImpl<'a>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<Self, Box<dyn Error>> {
        let win = session.connection().generate_id()?;
        let colormap = session.connection().generate_id()?;
        session.connection().create_colormap(
            ColormapAlloc::NONE,
            colormap,
            session.screen().root,
            session.visual_info().visual_id(),
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
            .background_pixel(session.screen().white_pixel);

        session.connection().create_window(
            session.visual_info().depth(),
            win,
            session.screen().root,
            x,
            y,
            width,
            height,
            0,
            WindowClass::INPUT_OUTPUT,
            session.visual_info().visual_id(),
            &win_aux,
        )?;

        let sfc = cairo::XCBSurface::create(
            session.cairo().cairo_connection(),
            &cairo::XCBDrawable(win),
            session.cairo().cairo_visual_type(),
            width.into(),
            height.into(),
        )
        .unwrap();

        Ok(Self {
            window: win,
            surface: sfc,
        })
    }

    fn draw(&self) -> Result<(), Box<dyn Error>> {
        let cr = cairo::Context::new(&self.surface)?;

        cr.set_source_rgb(0.5, 0.5, 1.0);
        cr.paint()?;

        self.surface.flush();
        Ok(())
    }
}
