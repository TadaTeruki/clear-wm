use log::info;
use x11rb::{
    connection::{Connection, RequestConnection},
    protocol::{
        render::{ConnectionExt, PictType},
        xproto::Visualid,
    },
    reexports::x11rb_protocol::protocol::render,
    xcb_ffi::{ReplyError, XCBConnection},
};
use zym_config::Config;
use zym_controller::start_session;
use zym_logger::WmLogger;
use zym_session::{
    repository::{cairo::WmCairoRepository, visual::WmVisualRepository},
    session::WmSession,
};

static LOGGER: WmLogger = WmLogger;

fn main() {
    let config = Config::load().expect("failed to load configs");
    LOGGER
        .init(config.build_config())
        .expect("failed to load a logger");

    let (connection, screen_num) = XCBConnection::connect(None).unwrap();
    info!("established connection to X server");

    let (depth, visual_id) = choose_visual(&connection, screen_num).unwrap();
    let mut visual_type = query_visual_type(&connection, visual_id);

    let cairo_connection =
        unsafe { cairo::XCBConnection::from_raw_none(connection.get_raw_xcb_connection() as _) };
    let cairo_visual_type =
        unsafe { cairo::XCBVisualType::from_raw_none(&mut visual_type as *mut _ as _) };

    let visual_info = WmVisualRepository::new(depth, visual_id);

    let cairo_session = WmCairoRepository::new(&cairo_connection, &cairo_visual_type);

    let session = WmSession::new(
        &connection,
        screen_num,
        &visual_info,
        &cairo_session,
        config.wm_config(),
    )
    .unwrap()
    .init_as_wm()
    .unwrap();

    start_session(&session);
}

// borrowed from: github.com/psychon/x11rb/cairo-example/src/main.rs
fn choose_visual(conn: &XCBConnection, screen_num: usize) -> Result<(u8, Visualid), ReplyError> {
    let depth = 32;
    let screen = &conn.setup().roots[screen_num];

    let has_render = conn
        .extension_information(render::X11_EXTENSION_NAME)?
        .is_some();
    if has_render {
        let formats = conn.render_query_pict_formats()?.reply()?;
        let format = formats
            .formats
            .iter()
            .filter(|info| (info.type_, info.depth) == (PictType::DIRECT, depth))
            .filter(|info| {
                let d = info.direct;
                (d.red_mask, d.green_mask, d.blue_mask, d.alpha_mask) == (0xff, 0xff, 0xff, 0xff)
            })
            .find(|info| {
                let d = info.direct;
                (d.red_shift, d.green_shift, d.blue_shift, d.alpha_shift) == (16, 8, 0, 24)
            });
        if let Some(format) = format {
            if let Some(visual) = formats.screens[screen_num]
                .depths
                .iter()
                .flat_map(|d| &d.visuals)
                .find(|v| v.format == format.id)
            {
                return Ok((format.depth, visual.visual));
            }
        }
    }
    Ok((screen.root_depth, screen.root_visual))
}

fn query_visual_type(
    connection: &XCBConnection,
    visual_id: u32,
) -> Option<&x11rb::protocol::xproto::Visualtype> {
    for root in &connection.setup().roots {
        for depth in &root.allowed_depths {
            for visual in &depth.visuals {
                if visual.visual_id == visual_id {
                    return Some(visual);
                }
            }
        }
    }
    None
}
