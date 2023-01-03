use crate::event_handler::types::WmEventHandler;
use log::error;

pub fn start_session(handler: &mut WmEventHandler) {
    loop {
        if let Err(err) = handler.handle_event() {
            error!("{}", err);
        }
    }
}
