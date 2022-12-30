mod handler;

use std::error::Error;

use log::{error, info};
use x11rb::protocol::Event;
use zym_session::common::{ClientSessionImpl, EventSessionImpl};

use crate::handler::{
    button_press::handle_button_press, button_release::handle_button_release,
    configure_request::handle_configure_request, enter_notify::handle_enter_notify,
    expose::handle_expose, map_request::handle_map_request, motion_notify::handle_motion_notify,
    unmap_notify::handle_unmap_notify,
};

pub fn start_session<'a, S: ClientSessionImpl<'a> + EventSessionImpl<'a>>(session: &mut S) {
    loop {
        if let Err(err) = event_loop(session) {
            error!("{}", err);
        }
    }
}

pub fn event_loop<'a, S: ClientSessionImpl<'a> + EventSessionImpl<'a>>(
    session: &mut S,
) -> Result<(), Box<dyn Error>> {
    let event = session.wait_for_event()?;
    let mut event_option = Some(event);

    while let Some(event) = event_option {
        info!("X event - {:?}", event);

        match event {
            Event::ButtonPress(event) => handle_button_press(session, &event)?,
            Event::ButtonRelease(event) => handle_button_release(session, &event)?,
            Event::ConfigureRequest(event) => handle_configure_request(session, &event)?,
            Event::EnterNotify(event) => handle_enter_notify(session, &event)?,
            Event::Expose(event) => handle_expose(session, &event)?,
            Event::MapRequest(event) => handle_map_request(session, &event)?,
            Event::MotionNotify(event) => handle_motion_notify(session, &event)?,
            Event::UnmapNotify(event) => handle_unmap_notify(session, &event)?,
            Event::Error(err) => error!("{:?}", err),
            _ => {}
        }
        event_option = session.poll_for_event()?;
    }
    Ok(())
}
