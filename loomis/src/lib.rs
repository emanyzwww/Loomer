//! Loomis: a Rust desktop UI framework.
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
//! use loomis::prelude::*;
//!
//! fn main() -> loomis::Result<()> {
//!     loomis::run(WindowOptions::new("Hello").size(800.0, 600.0))
//! }
//! ```

pub use loomis_core::geometry::Size;
pub use loomis_core::window::WindowOptions;
pub use loomis_runtime::{Error, Result, run};

/// Items intended for a glob import: `use loomis::prelude::*;`.
pub mod prelude {
    pub use crate::{Size, WindowOptions, run};
}
