use wm_listener::{
    event_listener::traits::EventListenerImpl, event_reflector::traits::EventReflectorImpl,
};
use wm_usecase::traits::ClientUseCaseImpl;

pub struct WmEventHandler<'a> {
    pub(super) client_usecase: &'a mut dyn ClientUseCaseImpl<'a>,
    pub(super) event_listener: &'a mut dyn EventListenerImpl<'a>,
    pub(super) event_reflector: &'a mut dyn EventReflectorImpl<'a>,
}

impl<'a> WmEventHandler<'a> {
    pub fn new(
        client_usecase_: &'a mut dyn ClientUseCaseImpl<'a>,
        event_listener_: &'a mut dyn EventListenerImpl<'a>,
        event_reflector_: &'a mut dyn EventReflectorImpl<'a>,
    ) -> Self {
        Self {
            client_usecase: client_usecase_,
            event_listener: event_listener_,
            event_reflector: event_reflector_,
        }
    }
}
