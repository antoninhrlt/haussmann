// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Widgets are items to be placed on a drawable surface and managed by layouts.

use crate::{
    graphics::colours::RGBA,
    ToAny,
};

mod button;
mod container;
mod image;
mod label;
mod layout;
mod surface;
mod view;

pub use button::Button;
pub use container::Container;
pub use image::Image;
pub use label::Label;
pub use layout::Layout;
pub use surface::Surface;
pub use view::View;

pub trait Widget: DebugWidget + ToAny {
    fn build(&self) -> Box<dyn Widget>;
    fn colour(&self) -> RGBA;
}

/// Automatically implemented by the macro [`dynamic_widget`](crate::dynamic_widget).
pub trait DebugWidget: std::fmt::Debug {
    /// Should not be overridden
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Implements what it is needed to make a widget a clean `dyn Widget` to be
/// inserted in layouts etc...
///
/// The `t` argument is the type of the widget. It can be a [`Button`], a
/// [`Label`], ...
///
/// To use this trait, the [`ToAny`], [`Widget`] and [`DebugWidget`] traits must
/// be imported in the usage context.
macro_rules! dynamic_widget {
    ($t:ty) => {
        impl ToAny for $t {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        impl From<$t> for Box<dyn Widget> {
            fn from(value: $t) -> Self {
                Box::new(value)
            }
        }

        impl DebugWidget for $t {}
    };
}

pub(crate) use dynamic_widget;

/// Same as [`dynamic_widget`] but for controller widgets since controllers 
/// have a generic parameters as following : `<T: Widget + 'static>`.
/// 
/// Also, the [`ToAny`] trait implementation returns the widget as 
/// [`std::any::Any`] and not the controller itself.
macro_rules! dynamic_controller {
    ($t:ty) => {
        impl<T: Widget + 'static> ToAny for $t {
            fn as_any(&self) -> &dyn std::any::Any {
                self.widget.as_any()
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
        
        impl<T: Widget + 'static> From<$t> for Box<dyn Widget> {
            fn from(value: $t) -> Self {
                Box::new(value)
            }
        }
        
        impl<T: Widget> DebugWidget for $t {}        
    };
}

pub(crate) use dynamic_controller;

/// Creates a vector of dynamic widgets from a series of widgets, no matter
/// their type as long as they implement the [`Widget`] trait.
///
/// To use this trait, the [`Widget`] trait must be imported in the usage
/// context.
#[macro_export]
macro_rules! widgets {
    () => {
        vec![]
    };

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
