// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{graphics::{Shape, shapes, colours::RGBA, Size}, Border, Radius, ToAny};
use super::{Widget, DebugWidget, Label};

/// Button widget, without text.
#[derive(Debug, Clone)]
pub struct Button {
    size: Size,
    colour: RGBA,
    pub radius: Radius,
    pub borders: Option<[Border; 4]>,
}

crate::dynamic_widget!(Button);

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
