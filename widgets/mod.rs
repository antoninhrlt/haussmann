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

pub mod buttons;
mod image;
mod label;
pub mod layout;

pub use buttons::Button;
pub use image::Image;
pub use label::Label;
pub use layout::Layout;

/// The simplest functionalities and property getters of any widget.
/// 
/// Every structure implementing the `Widget` trait must implement the following 
/// functions :
/// - `new`: Constructor with all the values.
/// - `simple`: Constructor for the simplest widget possible.
/// - Other constructors like `rounded` if a `radius` can be added or `bordered`
/// if `borders` can be added.
pub trait Widget: DebugWidget + ToAny {
    /// Returns the drawable shapes of the widget. Most widgets return a vector
    /// with only one shape in, the only widgets returning more than one shape
    /// are layouts.
    /// 
    /// If the returned vector is empty, it means there is no shape to draw. It 
    /// can be because it's not a surfaced widget, like a `Label`. The widget 
    /// has to be renderer another way. However, the widget must have a size.
    fn shapes(&self) -> Vec<Shape> {
        vec![]
    }

    /// Returns the larger size possible of the widget, from its shapes.
    /// 
    /// Nothing of the widget can be out of this zone.
    fn size(&self) -> Size;
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
