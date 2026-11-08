//! Application runtime.
//!
//! The runtime connects the platform layer to an application: it opens a window, runs the event
//! loop, and owns the callbacks the platform invokes. Everything above it talks to this crate
//! instead of reaching the platform layer directly.
//!
//! # Items
//!
//! | Item | Role |
//! |---|---|
//! | [`run`] | Opens a window and blocks until it closes |
//! | [`Error`] | Failures raised while running the application |
//! | [`Result`] | Result type used across the crate |
//!
//! # Layering
//!
//! ```text
//! loomer (facade) ──> loomer-runtime ──> loomer-platform
//!                            └─────────> loomer-core (types)
//! ```
//!
//! No type from the platform layer appears in a public signature except through [`Error`].

pub mod error;
pub mod runner;

pub use error::{Error, Result};
pub use runner::run;
