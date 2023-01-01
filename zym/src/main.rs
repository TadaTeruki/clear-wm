use log::info;
use x11rb::{
    connection::Connection,
    protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt as _, EventMask},
    xcb_ffi::XCBConnection,
};
use zym_config::Config;
use zym_controller::{handler::WmHandler, start_listener};
use zym_listener::event_listener::WmEventListener;
use zym_logger::WmLogger;
use zym_manager::client::manager::WmClientManager;
use zym_model::entity::visual::WmVisual;
use zym_usecase::client::WmClientUseCase;

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

    let visual = WmVisual::new(&connection, screen).unwrap();

    let mut client_manager = WmClientManager::new(&connection, screen, &visual, config.wm_config());
    let mut client_usecase = WmClientUseCase::new(&mut client_manager);
    let mut client_handler = WmHandler::new(&mut client_usecase);

    let mut listener = WmEventListener::new(&connection).unwrap();

    start_listener(&mut listener, &mut client_handler);
}
