// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

#[path = "../widgets/mod.rs"]
pub mod widgets;

pub mod graphics;

mod align;
mod border;
mod font;
mod overflow;
mod radius;
mod theme;

pub use align::{ Align, TextAlign };
pub use border::Border;
pub use font::*;
pub use overflow::Overflow;
pub use radius::Radius;
pub use theme::Theme;

pub use widgets::{ Widget, DebugWidget };

pub trait ToAny {
    fn as_any(&self) -> &dyn std::any::Any;
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
