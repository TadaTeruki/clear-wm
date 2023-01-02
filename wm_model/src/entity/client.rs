pub type ClientID = u32;

#[derive(Debug, Clone, Copy)]
pub enum WindowType {
    ComposedApp,
    UncomposedApp,
    Frame,
}
