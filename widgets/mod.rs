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
/// 
/// It's heavily recommanded to create another constructor called `all()` with 
/// all the options possible. Then, each constructor shall call `all()` with 
/// their values, and `None` values for the unwanted options.
pub trait Widget: DebugWidget + ToAny {
    /// Returns the drawable shapes of the widget.
    /// 
    /// If the returned vector is empty, it means there is no shape to draw. It 
    /// can be because it's not a surfaced widget like a `Label`. The widget has 
    /// to be renderer another way. However, the widget must have a size.
    fn shapes(&self) -> Vec<Shape> {
        vec![]
    }

    /// Returns the larger size possible of the widget, from its shapes.
    /// 
    /// Nothing of the widget can be out of this zone.
    fn size(&self) -> Size;
}

pub trait CreateWidget: Widget {
    /// Creates a widget without corner radius nor borders.
    fn new(size: Size, colour: RGBA) -> Self;

    /// Creates a widget with a corner radius.
    fn rounded(size: Size, colour: RGBA, radius: Radius) -> Self;

    /// Creates a widget with borders.
    fn bordered(size: Size, colour: RGBA, borders: [Border; 4]) -> Self;
}

/// Implements the `Debug` trait for dynamic `Widget` objects.
impl std::fmt::Debug for dyn Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Calls `DebugWidget::fmt`.
        self.fmt(f)
    }
}

/// Automatically implemented by the macro `dynamic_widget`.
/// 
/// Prints an object which implements the `Widget` trait.
pub trait DebugWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

#[macro_export]
macro_rules! dynamic_widget {
    ($t:ty) => {
        impl ToAny for $t {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        
        impl From<$t> for Box<dyn Widget> {
            fn from(value: $t) -> Self {
                Box::new(value)
            }
        }

        impl DebugWidget for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

#[macro_export]
macro_rules! widgets {
    ($first:expr $(, $widget:expr) *) => {
        {
            let widgets: Vec<Box<dyn Widget>> = vec![$first.into(), $($widget.into()),*];
            widgets
        }
    };
}
