//! Components and the per-render element description.
//!
//! This module defines the two foundational types of the framework:
//!
//! | Type | Role | Lifetime |
//! |---|---|---|
//! | [`Component`] | Reusable unit that owns state and describes the UI | Lives across frames |
//! | [`Element`] | One-shot **description** of the UI | Recreated on every render |
//!
//! The split matters: state belongs to the component, while an [`Element`] is only a transient
//! value produced by [`Component::view`]. Nothing inside an element is expected to persist between
//! two renders.
//!
//! # Data flow
//!
//! ```text
//! Component (owns state)
//!     │ view()
//!     ▼
//! Element (UI description)  ──> layout ──> drawing
//! ```
//!
//! Components only *describe* their UI. How the tree gets laid out and drawn, and when a component
//! has to be rendered again, is decided by `loomer-runtime`.

/// A reusable UI unit.
///
/// Implementors provide [`view`](Component::view): read the component's own state and return a
/// description of the UI. The runtime calls it whenever the component needs to be rendered, which
/// may happen very often, so it must not have side effects.
///
/// # Where state lives
///
/// The component's fields are its state. State has to survive across renders, so it **must not be
/// created inside `view()`** — doing so would reset it on every render. Create it once, when the
/// component is constructed.
///
/// # Examples
///
/// ```
/// use loomer_core::element::{Component, Element};
///
/// struct Hello;
///
/// impl Component for Hello {
///     fn view(&self) -> Element {
///         Element::empty() // nothing is drawn
///     }
/// }
/// ```
///
/// # Why `'static`
///
/// Components are stored in the element tree as `Box<dyn Component>`, and the tree outlives any
/// single call frame. A component therefore cannot borrow temporary data. State that has to be
/// shared must be held through an owned handle (such as `Rc`), never through a reference.
pub trait Component: 'static {
    /// Describes what this component looks like in its current state.
    ///
    /// - Takes `&self`: state must not be modified while rendering.
    /// - May be called many times: do not perform I/O, log, or create state here.
    fn view(&self) -> Element;
}

/// A one-shot description of the UI.
///
/// Produced by [`Component::view`] and consumed by the runtime for layout and drawing. It is **not
/// a persistent object**: a new element is created on every render, so it must not be expected to
/// remember anything.
pub struct Element {
    /// The root component.
    pub(crate) root: Option<Box<dyn Component>>,
}

impl Element {
    /// Creates an element that draws nothing.
    ///
    /// Useful while no widgets exist, or for conditional rendering where a branch should produce
    /// no visual output.
    ///
    /// ```
    /// use loomer_core::element::Element;
    ///
    /// let element = Element::empty(); // draws nothing
    /// ```
    pub fn empty() -> Self {
        Element { root: None }
    }

    /// Takes the stored component out and leaves an empty element behind.
    ///
    /// > **Framework-internal.** Not part of the public API.
    /// >
    /// > `loomer-runtime` needs the component instance back in order to drive rendering.
    /// > Rust does not allow moving a field out of `&mut self`, so it is taken explicitly.
    pub fn take_component(&mut self) -> Option<Box<dyn Component>> {
        self.root.take()
    }
}

/// Converts any component into an [`Element`].
///
/// This lets the framework convert component values without special-casing each type:
///
/// ```
/// use loomer_core::element::{Component, Element};
///
/// struct Hello;
/// impl Component for Hello {
///     fn view(&self) -> Element { Element::empty() }
/// }
///
/// // Hello -> Element
/// let element: Element = Hello.into();
/// ```
impl<C: Component> From<C> for Element {
    fn from(component: C) -> Self {
        Element {
            root: Some(Box::new(component)),
        }
    }
}
