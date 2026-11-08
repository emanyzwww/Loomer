//! Errors raised while running the application.

/// A failure raised while starting or running the application.
///
/// Platform failures are wrapped rather than flattened, so the stage that failed stays visible to
/// callers.
#[derive(Debug)]
pub enum Error {
    /// The platform layer failed to bring up or run the window.
    Platform(loomer_platform::Error),
}

/// Result type used by the runtime.
pub type Result<T> = std::result::Result<T, Error>;
