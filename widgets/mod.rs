// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    graphics::{
        Size, 
        Shape, 
    }, 
    ToAny
};

mod button;
mod container;
mod image;
mod label;
pub mod layout;

pub use button::Button;
pub use container::Container;
pub use image::Image;
pub use label::Label;
pub use layout::Layout;

/// The simplest functionalities and property getters of any widget.
pub trait Widget: DebugWidget + ToAny {
    /// Returns the widget as a shape of `size`. If it's a widget containing 
    /// other widgets, it does not return its children but only itself as a 
    /// shape.
    /// 
    /// ## Note
    /// When the widget is actually a `Container`, the `size` parameter is its 
    /// own size.
    fn shape(&self, size: Size) -> Shape;
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

/// Implements what it is needed to make this widget a clean `dyn Widget` to be 
/// inserted in layouts etc...
/// 
/// > The `t` argument is the type of the widget. It can be a `Button`, 
/// a `Label`, ...
/// 
/// To use this trait, the `ToAny`, `Widget`, `DebugWidget` traits must be 
/// imported in the usage context.
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

/// Creates a vector of dynamic widgets from a series of widgets, no matter 
/// their type as long as they implement the `Widget` trait.
/// 
/// To use this trait, the `Widget` trait must be imported in the usage context.
#[macro_export]
macro_rules! widgets {
    ($first:expr $(, $widget:expr) *,) => {
        widgets![$first, $($widget),*]
    };

    ($first:expr $(, $widget:expr) *) => {
        // Code block returning a vector of boxes of dynamic widget.
        {
            // The type annotation here is very important. It transforms the 
            // widget boxes into boxes of dynamic widget.
            let widgets: Vec<Box<dyn Widget>> = vec![
                $first.into(), 
                $($widget.into()),*
            ];
            widgets
        }
    };
}