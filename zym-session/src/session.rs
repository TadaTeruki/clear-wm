use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt as _, *};
use x11rb::xcb_ffi;
use zym_config::WmConfig;
use zym_model::common::session::{CairoRepositoryImpl, SessionImpl, VisualRepositoryImpl};

use x11rb::errors::ReplyError;
use x11rb::protocol::ErrorKind;

use thiserror::Error;

use crate::repository::cairo::WmCairoRepository;
use crate::repository::visual::WmVisualRepository;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("another wm is running")]
    AnotherWMIsRunning,

    #[error("unexpected error: {0}")]
    Unexpected(String),
}
pub struct WmSession<'a> {
    connection: &'a xcb_ffi::XCBConnection,
    screen: &'a Screen,
    visual_repository: &'a WmVisualRepository,
    cairo_repository: &'a WmCairoRepository<'a>,
    config: &'a WmConfig,
}

impl<'a> WmSession<'a> {
    pub fn new(
        connection_: &'a xcb_ffi::XCBConnection,
        screen_num: usize,
        visual_repository_: &'a WmVisualRepository,
        cairo_: &'a WmCairoRepository,
        config_: &'a WmConfig,
    ) -> Result<Self, ReplyError> {
        let screen_: &Screen = &connection_.setup().roots[screen_num];

        Ok(Self {
            connection: connection_,
            screen: screen_,
            visual_repository: visual_repository_,
            cairo_repository: cairo_,
            config: config_,
        })
    }

    pub fn init_as_wm(self) -> Result<Self, SessionError> {
        let aux = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY);

        match self
            .connection
            .change_window_attributes(self.screen.root, &aux)
        {
            Ok(result) => match result.check() {
                Ok(_) => {}
                Err(err) => {
                    if let ReplyError::X11Error(ref x11err) = err {
                        if x11err.error_kind == ErrorKind::Access {
                            return Err(SessionError::AnotherWMIsRunning);
                        }
                    }

                    return Err(SessionError::Unexpected(format!("{:?}", err)));
                }
            },
            Err(err) => {
                return Err(SessionError::Unexpected(err.to_string()));
            }
        }

        Ok(self)
    }
}

impl<'a> SessionImpl<'a> for WmSession<'a> {
    fn connection(&self) -> &'a xcb_ffi::XCBConnection {
        self.connection
    }

    fn screen(&self) -> &'a Screen {
        self.screen
    }

    fn config(&self) -> &'a WmConfig {
        self.config
    }

    fn visual_repository(&self) -> &'a dyn VisualRepositoryImpl {
        self.visual_repository
    }

    fn cairo_repository(&self) -> &'a dyn CairoRepositoryImpl<'a> {
        self.cairo_repository
    }
}
