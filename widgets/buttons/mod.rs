// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! This module contains all sort of buttons.
//! 
//! The `Button` widget is accessible from the `widgets` module directly.

mod text;
mod image;

pub use text::TextButton;
pub use image::ImageButton;

use crate::{graphics::{Shape, shapes, colours::RGBA, Size}, Border, Radius, ToAny};
use super::{Widget, DebugWidget};

#[derive(Debug, Clone)]
pub struct Button {
    size: Size,
    colour: RGBA,
    pub radius: Radius,
    pub borders: Option<[Border; 4]>,
}

crate::dynamic_widget!(Button);

impl From<&ImageButton> for Button {
    fn from(value: &ImageButton) -> Self {
        Self {
            size: value.size(),
            colour: value.colour,
            radius: value.radius,
            borders: value.borders,
        }
    }
}

impl From<&TextButton> for Button {
    fn from(value: &TextButton) -> Self {
        Self {
            size: value.size(),
            colour: value.colour,
            radius: value.radius,
            borders: value.borders,
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            size: [0, 0],
            colour: RGBA::new(0, 0, 0, 0),
            radius: Radius::new(0.0),
            borders: None,
        }
    }
}

impl Button {
    /// Creates a button with a `radius` and `borders`.
    pub fn new(size: Size, colour: RGBA, radius: Radius, borders: [Border; 4]) -> Self {
        Self {
            size,
            colour,
            radius,
            borders: Some(borders),
        }
    }

    /// Creates the simplest button possible, without radius nor borders.
    pub fn simple(size: Size, colour: RGBA) -> Self {
        Self {
            size,
            colour,
            ..Self::default()
        }
    }

    /// Creates a button with a `radius` but no `borders`.
    pub fn rounded(size: Size, colour: RGBA, radius: Radius) -> Self {
        Self {
            size,
            colour,
            radius,
            ..Self::default()
        }
    }

    /// Creates a button with `borders` but no `radius`.
    pub fn bordered(size: Size, colour: RGBA, borders: [Border; 4]) -> Self {
        Self {
            size,
            colour,
            borders: Some(borders),
            ..Self::default()
        }
    }
}

impl Widget for Button {
    /// Returns the drawable shapes of the widget.
    fn shapes(&self) -> Vec<Shape> {
        vec![
            shapes::Builder::new()
                .rectangle(self.size, self.borders)
                .fill(self.colour)
                .round(self.radius)
                .finish()
        ]
    }

    fn size(&self) -> Size {
        self.size
    }
}

/// Creates a button structure with a specific field such as a label, an 
/// image...
/// 
/// However, the `Default` and `Widget` traits must implemented manually. 
#[macro_export]
macro_rules! create_button {
    ($id:ident, $x:ident: $xt:ty) => {
        #[derive(Debug, Clone)]
        pub struct $id {
            size: Size,
            pub colour: RGBA,
            pub $x: $xt,
            pub radius: Radius,
            pub borders: Option<[Border; 4]>,
        }

        crate::dynamic_widget!($id);

        impl $id {
            /// Creates an $x button with a `radius` and `borders`.
            pub fn new(size: Size, colour: RGBA, $x: $xt, radius: Radius, borders: [Border; 4]) -> Self {
                Self {
                    size,
                    colour,
                    $x,
                    radius,
                    borders: Some(borders),
                }
            }

            /// Creates the simplest button possible, without radius nor borders.
            pub fn simple(size: Size, colour: RGBA, $x: $xt) -> Self {
                Self {
                    size,
                    colour,
                    $x,
                    ..Self::default()
                }
            }

            /// Creates a button with a `radius` but no `borders`.
            pub fn rounded(size: Size, colour: RGBA, $x: $xt, radius: Radius) -> Self {
                Self {
                    size,
                    colour,
                    $x,
                    radius,
                    ..Self::default()
                }
            }

            /// Creates a text button with `borders` but no `radius`.
            pub fn bordered(size: Size, colour: RGBA, $x: $xt, borders: [Border; 4]) -> Self {
                Self {
                    size,
                    colour,
                    $x,
                    borders: Some(borders),
                    ..Self::default()
                }
            }
        }

    };
}
