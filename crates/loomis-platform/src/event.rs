//! Events emitted by the platform layer.
//!
//! The layer translates `winit` window events into [`Event`] values, so no `winit` type crosses
//! into the layers above.

use loomis_core::geometry::Size;

/// An event reported by the platform layer.
///
/// [`AppHandler::event`](crate::AppHandler::event) receives these values; events the layer
/// cannot map are dropped.
#[derive(Debug)]
pub enum Event {
    /// The window was resized; the payload is the new size in logical pixels.
    Resized(Size),
    // TODO: translate mouse, keyboard, focus, and DPI-change events as well.
}

/// Translates a `winit` window event into an [`Event`].
///
/// Returns [`None`] for events the platform layer does not expose. `scale` is the scale factor
/// captured when the window is created; physical dimensions are divided by it to produce logical
/// pixels, so a DPI change after creation is not reflected.
pub(crate) fn translate(event: &winit::event::WindowEvent, scale: f32) -> Option<Event> {
    match event {
        winit::event::WindowEvent::Resized(physical) => Some(Event::Resized(Size::new(
            physical.width as f32 / scale,
            physical.height as f32 / scale,
        ))),
        _ => None,
    }
}
