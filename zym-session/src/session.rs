use std::error::Error;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConfigureRequestEvent, Screen, Window};
use x11rb::protocol::Event;
use x11rb::xcb_ffi::XCBConnection;
use zym_config::WmConfig;

use x11rb::errors::ReplyError;
use zym_model::common::manager::ClientManagerImpl;
use zym_model::entity::visual::WmVisual;

use crate::common::SessionImpl;

pub struct WmSession<'a> {
    connection: &'a XCBConnection,
    screen: &'a Screen,
    visual: &'a WmVisual,
    config: &'a WmConfig,
    manager: &'a mut dyn ClientManagerImpl<'a>,
}

impl<'a> WmSession<'a> {
    pub fn new(
        connection_: &'a XCBConnection,
        screen_: &'a Screen,
        visual_: &'a WmVisual,
        config_: &'a WmConfig,
        manager_: &'a mut dyn ClientManagerImpl<'a>,
    ) -> Result<Self, ReplyError> {
        Ok(Self {
            connection: connection_,
            screen: screen_,
            visual: visual_,
            config: config_,
            manager: manager_,
        })
    }
}

impl<'a> SessionImpl<'a> for WmSession<'a> {
    fn wait_for_event(&self) -> Result<Event, Box<dyn Error>> {
        self.connection.flush()?;
        let event = self.connection.wait_for_event()?;
        Ok(event)
    }

    fn poll_for_event(&self) -> Result<Option<Event>, Box<dyn Error>> {
        let event = self.connection.poll_for_event()?;
        Ok(event)
    }

    fn compose_client(&mut self, window: Window) -> Result<(), Box<dyn Error>> {
        let client_id = self.manager.create(
            self.connection,
            self.screen,
            self.visual,
            self.config,
            window,
        )?;
        self.manager.map(self.connection, client_id, self.config)?;
        Ok(())
    }

    fn configure_window(&self, event: &ConfigureRequestEvent) -> Result<(), Box<dyn Error>> {
        self.manager.configure(self.connection, event)?;
        Ok(())
    }
}
