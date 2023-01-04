use std::error::Error;

use std::str;
use wm_model::{entity::client::WmClient, traits::property_manager::PropertyManagerImpl};
use x11rb::protocol::xproto::ConnectionExt;

use super::types::WmPropertyManager;

impl<'a> PropertyManagerImpl for WmPropertyManager<'a> {
    fn client_title(&self, client: &WmClient) -> Result<String, Box<dyn Error>> {
        let prop = self
            .connection
            .get_property(
                false,
                client.app,
                self.atoms._NET_WM_NAME,
                self.atoms.UTF8_STRING,
                0,
                std::u32::MAX,
            )?
            .reply()?;
        {
            let str = str::from_utf8(&prop.value)?;
            if !str.is_empty() {
                return Ok(str.to_string());
            }
        }
        let prop = self
            .connection
            .get_property(
                false,
                client.app,
                self.atoms.WM_NAME,
                self.atoms.STRING,
                0,
                std::u32::MAX,
            )?
            .reply()?;
        {
            let str = str::from_utf8(&prop.value)?;
            if !str.is_empty() {
                return Ok(str.to_string());
            }
        }
        Ok(String::from(""))
    }
}
