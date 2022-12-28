use x11rb::protocol::xproto::{ConnectionExt, SetMode, Window};

use self::frame::WmClientFrameWindow;
use crate::common::{
    session::SessionImpl,
    window::{DrawableImpl, WindowImpl},
};
use std::error::Error;

pub mod frame;

pub struct WmClient {
    app: Window,
    frame: WmClientFrameWindow,
}

impl<'a> WmClient {
    fn create(session: &dyn SessionImpl<'a>, app_: Window) -> Result<Self, Box<dyn Error>> {
        let client_config = &session.config().client;

        let app_geom = session.connection().get_geometry(app_)?.reply()?;

        let frame_ = {
            let border_width = client_config.frame.border_width;
            let titlebar_height = client_config.frame.titlebar_height;
            WmClientFrameWindow::create(
                session,
                app_geom.x - border_width as i16,
                app_geom.y - border_width as i16,
                app_geom.width + border_width * 2,
                app_geom.height + border_width * 2 + titlebar_height,
            )?
        };

        session.connection().grab_server()?;
        session
            .connection()
            .change_save_set(SetMode::INSERT, app_)?;
        session.connection().reparent_window(
            app_,
            frame_.window(),
            client_config.frame.border_width as i16,
            client_config.frame.titlebar_height as i16,
        )?;
        session.connection().ungrab_server()?;

        Ok(Self {
            app: app_,
            frame: frame_,
        })
    }

    pub fn map(&self, session: &dyn SessionImpl<'a>) -> Result<(), Box<dyn Error>> {
        session.connection().grab_server()?;
        session.connection().map_window(self.app)?;
        session.connection().map_window(self.frame.window())?;
        session.connection().ungrab_server()?;
        Ok(())
    }

    pub fn expose(&self) -> Result<(), Box<dyn Error>> {
        self.frame.draw()?;
        Ok(())
    }
}
