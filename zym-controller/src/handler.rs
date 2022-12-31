pub mod button_press;
pub mod button_release;
pub mod configure_request;
pub mod enter_notify;
pub mod expose;
pub mod map_request;
pub mod motion_notify;
pub mod unmap_notify;

use zym_usecase::common::ClientUseCaseImpl;

pub struct WmHandler<'a> {
    client_usecase: &'a mut dyn ClientUseCaseImpl<'a>,
}

impl<'a> WmHandler<'a> {
    pub fn new(client_usecase_: &'a mut dyn ClientUseCaseImpl<'a>) -> Self {
        Self {
            client_usecase: client_usecase_,
        }
    }
}
