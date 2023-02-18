// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    graphics::{
        Size, 
        Shape, 
        colours::RGBA
    }, 
    Radius, 
    Border, ToAny
};

mod button;
mod label;
pub mod layout;

pub use button::Button;
pub use label::Label;
pub use layout::Layout;

/// The simplest functionalities and property getters of any widget.
/// 
/// A widget contains one or more shapes. Its size is the larger size being a 
/// zone containing all the shapes.
pub trait Widget: DebugWidget + ToAny {

}

/// Implements the `Debug` trait for dynamic `Widget` objects.
impl std::fmt::Debug for dyn Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

/// Automatically implemented by the macro `dynamic_widget`.
pub trait DebugWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

/// Trait for widgets having a surface of shapes
/// 
/// It's heavily recommanded to create another constructor called `all()` with 
/// all the options possible. Then, each constructor shall call `all()` with 
/// their values, and `None` values for the unwanted options.
/// 
/// Here is an example :
/// ```rust
/// impl Widget for Foo {
///     /// Creates the simplest foo possible.
///     fn new(size: Size, colour: RGBA) -> Self {
///         // No borders are given
///         Self::all(size, colour, None)
///     }
///     
///     /// Creates the most complex foo possible.
///     fn all(size: Size, colour: RGBA, borders: Option<[Border; 4]>) -> Self {
///         Self {
///             size,
///             colour,
///             borders,
///         }
///     }
/// }
/// ```
pub trait SurfacedWidget: Widget {
    /// Creates a widget without corner radius nor borders.
    fn new(size: Size, colour: RGBA) -> Self;
    
    /// Creates a widget with a corner radius.
    fn rounded(size: Size, colour: RGBA, radius: Radius) -> Self;

    /// Creates a widget with borders.
    fn bordered(size: Size, colour: RGBA, borders: [Border; 4]) -> Self;

    /// Returns the drawable shapes of the widget.
    fn shapes(&self) -> Vec<Shape>;

    /// Returns the larger size possible of the widget, from its shapes.
    /// 
    /// No shape of the widget can be out of this size zone.
    fn size(&self) -> Size;
}
