//! The [`Color`] type and its basic constants.
//!
//! # Color space
//!
//! `Color` stores sRGB components directly as `f32` values and performs **no** linearization.
//! Conversion to linear space is deferred to the renderer, so this type stays cheap to copy and
//! compare.
//!
//! # Alpha
//!
//! `a` is straight (non-premultiplied) alpha: the color and alpha channels are independent, and
//! the renderer blends them as `src * a + dst * (1 - a)`. Do not premultiply when constructing a
//! [`Color`], or semi-transparent compositing will come out wrong.

/// An sRGB color with straight (non-premultiplied) alpha.
///
/// All components are `f32`. `r`, `g` and `b` are conventionally in `0.0..=1.0`;
/// `a` ranges from `0.0` (fully transparent) to `1.0` (opaque).
///
/// # Examples
///
/// ```
/// use loomis_core::color::Color;
///
/// let red = Color::rgba(1.0, 0.0, 0.0, 1.0);
/// assert_eq!(red.r, 1.0);
/// assert_eq!(red.g, 0.0);
///
/// // 50% black, useful as a scrim.
/// let scrim = Color::rgba(0.0, 0.0, 0.0, 0.5);
/// assert_eq!(scrim.a, 0.5);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    /// The red component.
    pub r: f32,
    /// The green component.
    pub g: f32,
    /// The blue component.
    pub b: f32,
    /// The alpha (opacity) component.
    pub a: f32,
}

impl Color {
    /// Opaque white.
    pub const WHITE: Self = Self::rgba(1.0, 1.0, 1.0, 1.0);

    /// Opaque black.
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);

    /// Creates a color from its four components.
    ///
    /// This is a `const fn`, so it can be used to define constants:
    ///
    /// ```
    /// use loomis_core::color::Color;
    ///
    /// const BACKGROUND: Color = Color::rgba(0.12, 0.12, 0.16, 1.0);
    /// assert_eq!(BACKGROUND.a, 1.0);
    /// ```
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    // TODO: Add these once actual drawing is in place, to avoid writing APIs that aren't used yet.
}
