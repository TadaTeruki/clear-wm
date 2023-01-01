pub mod handler;

use std::error::Error;

use handler::WmHandler;
use log::{error, info};
use x11rb::protocol::Event;
use zym_listener::common::EventListenerImpl;

pub fn start_listener<'a, EL: EventListenerImpl<'a>>(
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
    let event = listener.wait()?;
    let mut event_option = Some(event);

    while let Some(event) = event_option {
        info!("X event - {:?}", event);

        match event {
            Event::ButtonPress(event) => handler.handle_button_press(&event)?,
            Event::ButtonRelease(event) => handler.handle_button_release(&event)?,
            Event::ConfigureRequest(event) => handler.handle_configure_request(&event)?,
            Event::EnterNotify(event) => handler.handle_enter_notify(&event)?,
            Event::Expose(event) => handler.handle_expose(&event)?,
            Event::MapRequest(event) => handler.handle_map_request(&event)?,
            Event::MotionNotify(event) => handler.handle_motion_notify(&event)?,
            Event::UnmapNotify(event) => handler.handle_unmap_notify(&event)?,
            Event::Error(err) => error!("{:?}", err),
            _ => {}
        }
        event_option = listener.poll()?;
    }
    Ok(())
}
