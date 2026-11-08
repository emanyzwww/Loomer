//! The entry point and the platform callback bridge.
//!
//! [`run`] hands its `Runner` to the platform layer. The runner receives lifecycle callbacks and,
//! for now, only asks for the first frame.

use loomer_core::window::WindowOptions;
use loomer_platform::{AppHandler, Event, Window};

use crate::error::{Error, Result};

/// Opens a window and blocks until it closes.
///
/// The window is created by the platform layer, which then drives the handler until the window is
/// closed.
///
/// # Errors
///
/// Returns [`Error::Platform`] when the platform layer cannot create or run the event loop.
pub fn run(options: WindowOptions) -> Result<()> {
    loomer_platform::run(Box::new(Runner), options).map_err(Error::Platform)
}

/// Receives platform callbacks for one application.
///
/// No application state exists yet, so the callbacks only keep the event loop alive and request
/// the first frame.
struct Runner;

impl AppHandler for Runner {
    /// Requests the first frame once the window exists.
    fn created(&mut self, window: &Window) {
        window.request_redraw();
    }

    /// Ignores platform events; none are consumed.
    fn event(&mut self, _event: Event) {}

    /// Called for each frame; produces no output.
    fn redraw(&mut self, _window: &Window) {}
}
