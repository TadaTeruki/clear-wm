/*
use std::error::Error;

use cairo::XCBSurface;
use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ColormapAlloc, ConnectionExt, CreateWindowAux, EventMask, Window, WindowClass,
    },
};

use crate::repository::connection::CairoConnectionRepository;

pub struct WmDrawable {
    window: Window,
    surface: XCBSurface,
}

impl<'a> WmDrawable {
    pub fn new(
        cs: &dyn CairoConnectionRepository<'a>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<Self, Box<dyn Error>> {
        let win = cs.connection().generate_id()?;
        let colormap = cs.connection().generate_id()?;
        cs.connection().create_colormap(
            ColormapAlloc::NONE,
            colormap,
            cs.screen().root,
            cs.visual_id(),
        )?;

        let win_aux = CreateWindowAux::new()
            .event_mask(EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY)
            .background_pixel(x11rb::NONE)
            .border_pixel(cs.screen().black_pixel)
            .colormap(colormap);

        cs.connection().create_window(
            cs.depth(),
            win,
            cs.screen().root,
            x,
            y,
            width,
            height,
            0,
            WindowClass::INPUT_OUTPUT,
            cs.visual_id(),
            &win_aux,
        )?;

        let sfc = cairo::XCBSurface::create(
            cs.cairo_connection(),
            &cairo::XCBDrawable(win),
            cs.cairo_visual_type(),
            width.into(),
            height.into(),
        )
        .unwrap();

        Ok(Self {
            window: win,
            surface: sfc,
        })
    }
}
*/
