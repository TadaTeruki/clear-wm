use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("another wm is running")]
    AnotherWMIsRunning,

    #[error("unexpected error: {0}")]
    Unexpected(String),
}
