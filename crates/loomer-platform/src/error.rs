//! Errors raised by the platform layer.
//!
//! Each variant records the stage that failed and carries the backend error unchanged.

/// A failure while bringing up or running the platform layer.
#[derive(Debug)]
pub enum Error {
    /// Creating or running the event loop failed.
    EventLoop(winit::error::EventLoopError),
    /// Creating the window failed.
    Window(winit::error::OsError),
}

impl From<winit::error::EventLoopError> for Error {
    fn from(e: winit::error::EventLoopError) -> Self {
        Error::EventLoop(e)
    }
}

impl From<winit::error::OsError> for Error {
    fn from(e: winit::error::OsError) -> Self {
        Error::Window(e)
    }
}
