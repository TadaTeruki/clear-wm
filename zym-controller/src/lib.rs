pub mod handler;

use std::error::Error;

use handler::WmHandler;
use log::{error, info, warn};
use x11rb::protocol::Event;
use zym_listener::common::EventListenerImpl;

pub fn start_session<'a, EL: EventListenerImpl<'a>>(
    listener: &mut EL,
    handler: &mut WmHandler<'a>,
) {
    loop {
        if let Err(err) = event_loop(listener, handler) {
            error!("{}", err);
        }
    }
}

pub fn event_loop<'a, EL: EventListenerImpl<'a>>(
    listener: &mut EL,
    handler: &mut WmHandler<'a>,
) -> Result<(), Box<dyn Error>> {
    warn!("waiting...");
    let event = listener.wait()?;
    let mut event_option = Some(event);

    while let Some(event) = event_option {
        info!("X event - {:?}", event);

        match event {
            Event::ButtonPress(ev) => handler.handle_button_press(&ev)?,
            Event::ButtonRelease(ev) => handler.handle_button_release(&ev)?,
            Event::ClientMessage(ev) => handler.handle_client_message(&ev)?,
            Event::ConfigureRequest(ev) => handler.handle_configure_request(&ev)?,
            Event::EnterNotify(ev) => handler.handle_enter_notify(&ev)?,
            Event::Expose(ev) => handler.handle_expose(&ev)?,
            Event::MapRequest(ev) => handler.handle_map_request(&ev)?,
            Event::MotionNotify(ev) => handler.handle_motion_notify(&ev)?,
            Event::UnmapNotify(ev) => handler.handle_unmap_notify(&ev)?,
            Event::Error(err) => error!("{:?}", err),
            _ => {}
        }
        event_option = listener.poll()?;
    }

    Ok(())
}
