use log::info;
use wm_config::Config;
use wm_controller::{event_handler::types::WmEventHandler, session::start_session};
use wm_listener::{
    event_listener::types::WmEventListener, event_reflector::types::WmEventReflector,
};
use wm_logger::WmLogger;
use wm_manager::{
    client_manager::types::manager::WmClientManager, cursor_manager::types::WmCursorManager,
    server_manager::types::WmServerManager,
};
use wm_model::entity::visual::WmVisual;
use wm_usecase::client_usecase::types::WmClientUseCase;
use x11rb::{
    connection::Connection,
    protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt as _, EventMask},
    xcb_ffi::XCBConnection,
};

static LOGGER: WmLogger = WmLogger;

fn main() {
    let config = Config::load().expect("failed to load configs");
    LOGGER
        .init(config.build_config())
        .expect("failed to load a logger");

    let (connection, screen_num) = XCBConnection::connect(None).unwrap();
    info!("established connection to X server");

    let screen = &connection.setup().roots[screen_num];

    let aux = ChangeWindowAttributesAux::default()
        .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY);
    connection
        .change_window_attributes(screen.root, &aux)
        .unwrap();

    let visual = WmVisual::new(screen).unwrap();

    let mut client_manager = WmClientManager::new(&connection, screen, &visual, config.wm_config());
    let mut cursor_manager = WmCursorManager::new(config.wm_config());
    let mut server_manager = WmServerManager::new(&connection);

    let mut client_usecase = WmClientUseCase::new(
        &mut client_manager,
        &mut cursor_manager,
        &mut server_manager,
    );

    let mut event_listener = WmEventListener::new(&connection).unwrap();
    let mut event_reflector = WmEventReflector::new(&connection).unwrap();
    let mut client_handler = WmEventHandler::new(
        &mut client_usecase,
        &mut event_listener,
        &mut event_reflector,
    );

    start_session(&mut client_handler);
}
