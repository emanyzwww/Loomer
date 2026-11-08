//! Platform layer: window creation, the event loop, and event translation.
//!
//! This crate is the only one in the workspace that depends on the windowing backend (`winit`). It
//! opens the window, drives the event loop, and translates backend events into Loomis events, so
//! that backend types never cross this boundary.
//!
//! # Items
//!
//! | Item | Role |
//! |---|---|
//! | [`run`] | Creates the event loop, opens one window, and blocks until it closes |
//! | [`AppHandler`] | Callback trait implemented by `loomis-runtime` |
//! | [`Window`] | Narrow handle over the backend window |
//! | [`Event`] | Backend-agnostic events passed to the handler |
//! | [`Error`] | Failures raised while creating the loop or the window |
//!
//! # Layering
//!
//! ```text
//! loomis-runtime ──> loomis-platform ──> winit
//!                          └──────────> loomis-core (sizes)
//! ```
//!
//! No type from the backend appears in a public signature of this crate except inside [`Error`],
//! where backend errors are carried unchanged.

pub mod app;
pub mod error;
pub mod event;
pub mod window;

pub use app::{AppHandler, run};
pub use error::Error;
pub use event::Event;
pub use window::Window;
