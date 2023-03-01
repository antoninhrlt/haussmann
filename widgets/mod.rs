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

/// Collection of the functions required to use and build widgets. All widgets 
/// implement it. 
/// 
/// Can be implemented with the macro derive 
/// [`#[derive(Widget)]`](haussmann_dev::widget).
/// 
/// The controllers also implement this trait since they are widgets, but it 
/// must be implemented thanks to 
/// [`#[derive(Controller)]`](haussmann_dev::controller) instead.
pub trait Widget: DebugWidget + ToAny {
    /// Builds the widget. The returned value will be transformed into a 
    /// [`Drawable`](crate::graphics::draw::Drawable) at [`View`] build.
    fn build(&self) -> Box<dyn Widget>;
    /// Returns the colour of the widget.
    fn colour(&self) -> RGBA;
}

/// Automatically implemented by the macro [`dynamic_widget`](crate::dynamic_widget).
pub trait DebugWidget: std::fmt::Debug {
    /// Should not be overridden
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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

    ($first:expr $(, $widget:expr)* $(,)*) => {
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
