//! Geometry primitives: sizes, points, and rectangles.
//!
//! The module defines [`Size`], [`Point`], and [`Rect`]. All three are `Copy` and cheap to pass by
//! value.
//!
//! # Units
//!
//! Every length in this module is a **logical pixel** (`f32`). Logical pixels are converted to
//! physical pixels at the boundary of the platform and rendering layers; that conversion never
//! reaches this module or anything above it.
//!
//! # Coordinate system
//!
//! The origin is the top-left corner of the window, `x` increases to the right, and `y` increases
//! downward.

/// A two-dimensional size.
///
/// The layout pass uses it to express how much space an element needs and how much space it has
/// been given. Once an element is placed, its size is carried by a [`Rect`].
///
/// # Examples
///
/// ```
/// use loomer_core::geometry::Size;
///
/// let available = Size::new(800.0, 600.0);
/// assert_eq!(available.width, 800.0);
/// assert_eq!(available.height, 600.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    /// The width.
    pub width: f32,
    /// The height.
    pub height: f32,
}

impl Size {
    /// The zero size.
    ///
    /// Serves as the initial value for "not measured yet". Equal to `Size::default()`.
    pub const ZERO: Self = Self {
        width: 0.0,
        height: 0.0,
    };

    /// Creates a size from a width and a height.
    ///
    /// This is a `const fn`, so it can be used to define constants:
    ///
    /// ```
    /// use loomer_core::geometry::Size;
    ///
    /// const DEFAULT_WINDOW: Size = Size::new(800.0, 600.0);
    /// assert_eq!(DEFAULT_WINDOW, Size::new(800.0, 600.0));
    /// ```
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// A two-dimensional point.
///
/// Used for positions and offsets. A [`Rect`] records its top-left corner as one.
///
/// # Examples
///
/// ```
/// use loomer_core::geometry::Point;
///
/// let p = Point { x: 12.0, y: 8.0 };
/// assert_eq!(p.x, 12.0);
/// assert_eq!(p.y, 8.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    /// The horizontal coordinate.
    pub x: f32,
    /// The vertical coordinate.
    pub y: f32,
}

/// An axis-aligned rectangle: a top-left corner plus a size.
///
/// Rectangles are produced by the layout pass, which computes a final rectangle for each element,
/// and consumed by the drawing pass when emitting primitives. The rectangle spans `x..x + width`
/// horizontally and `y..y + height` vertically.
///
/// # Examples
///
/// ```
/// use loomer_core::geometry::Rect;
///
/// let bounds = Rect { x: 16.0, y: 8.0, width: 200.0, height: 40.0 };
/// assert_eq!(bounds.x + bounds.width, 216.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    /// The horizontal coordinate of the top-left corner.
    pub x: f32,
    /// The vertical coordinate of the top-left corner.
    pub y: f32,
    /// The width.
    pub width: f32,
    /// The height.
    pub height: f32,
}
