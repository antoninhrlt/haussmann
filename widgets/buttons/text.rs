// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{graphics::{Shape, shapes, colours::RGBA, Size}, Border, Radius, ToAny};
use super::super::{Widget, DebugWidget, Label};

crate::create_button!(TextButton, label: Label);

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