//! Loomer: a Rust desktop UI framework.
//!
//! Application code depends on this crate only. The layers below it are private implementation
//! details and may be restructured without notice.
//!
//! # Surface
//!
//! | Item | Role |
//! |---|---|
//! | [`run`] | Opens a window and blocks until it is closed |
//! | [`WindowOptions`] | Describes the window to open |
//! | [`Size`] | Length unit used for window sizes |
//! | [`Error`] / [`Result`] | Outcome of starting an application |
//!
//! # Usage
//!
//! ```
//! use loomer::prelude::*;
//!
//! fn main() -> loomer::Result<()> {
//!     loomer::run(WindowOptions::new("Hello").size(800.0, 600.0))
//! }
//! ```

pub use loomer_core::geometry::Size;
pub use loomer_core::window::WindowOptions;
pub use loomer_runtime::{Error, Result, run};

/// Items intended for a glob import: `use loomer::prelude::*;`.
pub mod prelude {
    pub use crate::{Size, WindowOptions, run};
}
