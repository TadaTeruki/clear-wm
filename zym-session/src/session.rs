use crate::error::SessionError;
use log::{error, info};
use x11rb::connection::Connection;
use x11rb::protocol::Event;
use zym_domain::repository::session::SessionRepository;

use x11rb::errors::ReplyError;
use x11rb::protocol::xproto::*;
use x11rb::protocol::ErrorKind;
use zym_event::handler::button_press::ButtonPressHandler;
use zym_event::handler::button_release::ButtonReleaseHandler;
use zym_event::handler::configure_request::ConfigureRequestHandler;
use zym_event::handler::enter_notify::EnterNotifyHandler;
use zym_event::handler::expose::ExposeHandler;
use zym_event::handler::map_request::MapRequestHandler;
use zym_event::handler::motion_notify::MotionNotifyHandler;
use zym_event::handler::unmap_notify::UnmapNotifyHandler;

use std::error::Error;
pub struct WMSession<'a, C: Connection> {
    c: &'a C,
    s: &'a Screen,
}

impl<'a, C: Connection> WMSession<'a, C> {
    pub fn new(connection: &'a C, screen_num: usize) -> Self {
        let screen_: &'a Screen = &connection.setup().roots[screen_num];

        Self {
            c: connection,
            s: screen_,
        }
    }

    pub fn set_as_wm(&self) -> Result<(), SessionError> {
        let aux = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY);

        match self.c.change_window_attributes(self.s.root, &aux) {
            Ok(result) => match result.check() {
                Ok(_) => Ok(()),
                Err(err) => {
                    if let ReplyError::X11Error(ref x11err) = err {
                        if x11err.error_kind == ErrorKind::Access {
                            return Err(SessionError::AnotherWMIsRunning);
                        }
                    }

                    Err(SessionError::Unexpected(format!("{:?}", err)))
                }
            },
            Err(err) => Err(SessionError::Unexpected(err.to_string())),
        }
    }

    pub fn handle_event(&self) -> Result<(), Box<dyn Error>> {
        self.c.flush()?;
        let event = self.c.wait_for_event()?;
        let mut event_option = Some(event);
        while let Some(event) = event_option {
            info!("X event - {:?}", event);

            match event {
                Event::ButtonPress(event) => ButtonPressHandler::new(self, &event).execute()?,
                Event::ButtonRelease(event) => ButtonReleaseHandler::new(self, &event).execute()?,
                Event::ConfigureRequest(event) => {
                    ConfigureRequestHandler::new(self, &event).execute()?
                }
                Event::EnterNotify(event) => EnterNotifyHandler::new(self, &event).execute()?,
                Event::Expose(event) => ExposeHandler::new(self, &event).execute()?,
                Event::MapRequest(event) => MapRequestHandler::new(self, &event).execute()?,
                Event::MotionNotify(event) => MotionNotifyHandler::new(self, &event).execute()?,
                Event::UnmapNotify(event) => UnmapNotifyHandler::new(self, &event).execute()?,
                Event::Error(err) => error!("{:?}", err),
                _ => {}
            }
            event_option = self.c.poll_for_event()?;
        }
        Ok(())
    }
}

impl<'a, C: Connection> SessionRepository<'a, C> for WMSession<'a, C> {
    fn connection(&self) -> &'a C {
        self.c
    }

    fn screen(&self) -> &'a Screen {
        self.s
    }
}
