use std::error::Error;

use x11rb::{atom_manager, xcb_ffi::XCBConnection};

atom_manager! {
    pub AtomCollection: AtomCollectionCookie {
        WM_NAME,
        _NET_WM_NAME,
        UTF8_STRING,
        STRING,
    }
}

pub struct WmPropertyManager<'a> {
    pub(super) connection: &'a XCBConnection,
    pub(super) atoms: AtomCollection,
}

impl<'a> WmPropertyManager<'a> {
    pub fn new(connection_: &'a XCBConnection) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            connection: connection_,
            atoms: AtomCollection::new(connection_)?.reply()?,
        })
    }
}
