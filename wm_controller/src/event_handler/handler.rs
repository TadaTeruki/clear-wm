use wm_listener::traits::EventListenerImpl;
use wm_usecase::traits::ClientUseCaseImpl;

pub struct WmEventHandler<'a> {
    pub(super) client_usecase: &'a mut dyn ClientUseCaseImpl<'a>,
    pub(super) event_listener: &'a mut dyn EventListenerImpl<'a>,
}

impl<'a> WmEventHandler<'a> {
    pub fn new(
        client_usecase_: &'a mut dyn ClientUseCaseImpl<'a>,
        event_listener_: &'a mut dyn EventListenerImpl<'a>,
    ) -> Self {
        Self {
            client_usecase: client_usecase_,
            event_listener: event_listener_,
        }
    }
}
