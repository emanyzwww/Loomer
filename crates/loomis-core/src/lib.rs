//! Core types of the framework (`loomis-core`).
//!
//! This layer depends on no external crate and never touches windows, the GPU, or the file system.
//! It holds only the pure data and abstractions that every layer above it needs:
//!
//! | Module | Contents |
//! |---|---|
//! | [`color`] | Colors |
//! | [`element`] | Components and UI descriptions |
//! | [`geometry`] | Sizes, positions, and rectangles |
//! | [`window`] | Window creation options |
//!
//! # Position in the layering
//!
//! ```text
//! loomis (facade, the only public API)
//!  └─ loomis-runtime
//!      ├─ loomis-layout    ─┐
//!      ├─ loomis-render    ─┤
//!      ├─ loomis-platform  ─┼─> loomis-core (this crate, no dependencies)
//!      └─ loomis-widgets   ─┘
//! ```
//!
//! Dependencies point in one direction only: **this layer never knows what sits above it**.
//!
//! # Two conventions
//!
//! 1. **This layer is not a user-facing API.** Users reach them only through the `loomis` facade;
//!    internal paths such as `loomis_core::geometry::Size` may be restructured at any time.
//! 2. **Pure data and pure functions.** No I/O, no global state, no handles.
//!    Unit-testable without a window.

pub mod color;
pub mod element;
pub mod geometry;
pub mod window;
