// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{graphics::{Shape, shapes, colours::RGBA, Size}, Border, Radius, ToAny};
use super::{Widget, DebugWidget, Label};

#[derive(Debug, Clone)]
pub struct TextButton {
    size: Size,
    colour: RGBA,
    pub label: Label,
    pub radius: Radius,
    pub borders: Option<[Border; 4]>,
}

crate::dynamic_widget!(TextButton);

impl Widget for TextButton {
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

impl Default for TextButton {
    fn default() -> Self {
        Self {
            size: [0, 0],
            colour: RGBA::default(),
            label: Label::default(),
            radius: Radius::default(),
            borders: None,
        }
    }
}

impl TextButton {
    /// Creates a text button with a `radius` and `borders`.
    pub fn new(size: Size, colour: RGBA, label: Label, radius: Radius, borders: [Border; 4]) -> Self {
        Self {
            size,
            colour,
            label,
            radius,
            borders: Some(borders),
        }
    }

    /// Creates the simplest text button possible, without radius nor borders.
    pub fn simple(size: Size, colour: RGBA, label: Label) -> Self {
        Self {
            size,
            colour,
            label,
            ..Self::default()
        }
    }

    /// Creates a text button with a `radius` but no `borders`.
    pub fn rounded(size: Size, colour: RGBA, label: Label, radius: Radius) -> Self {
        Self {
            size,
            colour,
            label,
            radius,
            ..Self::default()
        }
    }

    /// Creates a text button with `borders` but no `radius`.
    pub fn bordered(size: Size, colour: RGBA, label: Label, borders: [Border; 4]) -> Self {
        Self {
            size,
            colour,
            label,
            borders: Some(borders),
            ..Self::default()
        }
    }
}
