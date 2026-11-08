//! Window creation options.
//!
//! The module defines [`WindowOptions`], the values needed to create a window: a title and an
//! initial size. Options are assembled with chained methods that consume and return `self`.

use crate::geometry::Size;

/// Options that describe a window before it is created.
///
/// A value starts from [`new`](WindowOptions::new) and is refined with builder methods such as
/// [`size`](WindowOptions::size); each one consumes and returns `Self`.
#[derive(Debug)]
pub struct WindowOptions {
    /// The window title.
    pub title: String,
    /// The initial window size.
    pub size: Size,
}

impl WindowOptions {
    /// Creates options from a title, with a default size of `800.0` by `600.0`.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            size: Size::new(800.0, 600.0),
        }
    }

    /// Sets the initial window size and returns the updated options.
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = Size::new(width, height);
        self
    }
}
