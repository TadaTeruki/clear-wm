use wm_config::WmConfig;
use wm_model::entity::cursor::WmCursorDragInfo;

pub struct WmCursorManager<'a> {
    pub(super) drag: Option<WmCursorDragInfo>,
    pub(super) config: &'a WmConfig,
}

impl<'a> WmCursorManager<'a> {
    pub fn new(config_: &'a WmConfig) -> Self {
        Self {
            drag: None,
            config: config_,
        }
    }
}
