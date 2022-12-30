use x11rb::protocol::xproto::Window;

pub type ClientID = u32;

#[derive(Debug, Clone, Copy)]
pub enum WindowType {
    App,
    Frame,
}

pub struct WmClient {
    pub client_id: ClientID,
    pub app: Window,
    pub frame: Window,
    pub frame_surface: cairo::XCBSurface,
}

impl WmClient {
    pub fn new(
        client_id_: ClientID,
        app_: Window,
        frame_: Window,
        frame_surface_: cairo::XCBSurface,
    ) -> Self {
        Self {
            client_id: client_id_,
            app: app_,
            frame: frame_,
            frame_surface: frame_surface_,
        }
    }
}

pub mod geometry {
    use zym_config::WmConfig;

    use crate::entity::geometry::Geometry;

    pub struct ClientGeometry {
        pub x: i16,
        pub y: i16,
        pub width: u16,
        pub height: u16,
    }

    impl ClientGeometry {
        pub fn from_app_absolute(
            x_: i16,
            y_: i16,
            width_: u16,
            height_: u16,
            config: &WmConfig,
        ) -> Self {
            let titlebar_height = config.client.frame.titlebar_height;

            Self {
                x: x_,
                y: y_ - titlebar_height as i16,
                width: width_,
                height: height_ + titlebar_height as u16,
            }
        }

        pub fn from_frame(x_: i16, y_: i16, width_: u16, height_: u16, config: &WmConfig) -> Self {
            let border_width = config.client.frame.border_width;

            Self {
                x: x_ + border_width as i16,
                y: y_ + border_width as i16,
                width: width_ - border_width as u16 * 2,
                height: height_ - border_width as u16 * 2,
            }
        }

        pub fn to_frame(&self, config: &WmConfig) -> Geometry {
            let border_width = config.client.frame.border_width;

            Geometry {
                x: self.x - border_width as i16,
                y: self.y - border_width as i16,
                width: self.width + border_width as u16 * 2,
                height: self.height + border_width as u16 * 2,
            }
        }

        pub fn to_app_relative(&self, config: &WmConfig) -> Geometry {
            let border_width = config.client.frame.border_width;
            let titlebar_height = config.client.frame.titlebar_height;

            Geometry {
                x: border_width as i16,
                y: border_width as i16 + titlebar_height as i16,
                width: self.width,
                height: self.height - titlebar_height as u16,
            }
        }
    }

    pub fn app_relative_position(config: &WmConfig) -> (i16, i16) {
        let border_width = config.client.frame.border_width;
        let titlebar_height = config.client.frame.titlebar_height;
        (
            border_width as i16,
            border_width as i16 + titlebar_height as i16,
        )
    }
}
