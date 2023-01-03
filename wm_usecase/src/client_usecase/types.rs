use wm_model::traits::{
    client_manager::ClientManagerImpl, collection_manager::CollectionManagerImpl,
    cursor_manager::CursorManagerImpl, server_manager::ServerManagerImpl,
};

pub struct WmClientUseCase<'a> {
    pub client_manager: &'a mut dyn ClientManagerImpl<'a>,
    pub cursor_manager: &'a mut dyn CursorManagerImpl,
    pub server_manager: &'a mut dyn ServerManagerImpl,
    pub collection_manager: &'a mut dyn CollectionManagerImpl,
}

impl<'a> WmClientUseCase<'a> {
    pub fn new(
        client_manager_: &'a mut dyn ClientManagerImpl<'a>,
        cursor_manager_: &'a mut dyn CursorManagerImpl,
        server_manager_: &'a mut dyn ServerManagerImpl,
        collection_manager_: &'a mut dyn CollectionManagerImpl,
    ) -> Self {
        Self {
            client_manager: client_manager_,
            cursor_manager: cursor_manager_,
            server_manager: server_manager_,
            collection_manager: collection_manager_,
        }
    }
}
