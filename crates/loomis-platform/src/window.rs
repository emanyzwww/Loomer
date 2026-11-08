//! A narrow handle over a created window.
//!
//! [`Window`] wraps the backend window and exposes only the operations the framework needs, so the
//! backend type stays inside this crate. Window *options* are plain data and live in
//! `loomis_core::window`; this module holds the live handle only.

/// A handle to a window created by [`run`](crate::run).
///
/// The platform layer owns the handle and lends it to the runtime through
/// [`AppHandler::created`](crate::AppHandler::created) and
/// [`AppHandler::redraw`](crate::AppHandler::redraw). Only a curated subset of the backend API is
/// exposed: request a redraw and change the title.
pub struct Window {
    /// The backend window.
    ///
    /// `pub(crate)` because the event loop needs operations that are not worth exposing, such as
    /// reading the scale factor.
    pub(crate) inner: winit::window::Window,
}

impl Window {
    /// Asks the platform to deliver a redraw request.
    ///
    /// The request is queued: [`AppHandler::redraw`](crate::AppHandler::redraw) runs on a later
    /// iteration of the event loop, not immediately.
    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }

    /// Replaces the text shown in the title bar.
    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }
}
