// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{graphics::{Shape, shapes, colours::RGBA, Size}, Border, Radius, ToAny};
use super::super::{Widget, DebugWidget, Image};

crate::create_button!(ImageButton, image: Image);

impl Default for ImageButton {
    fn default() -> Self {
        Self {
            size: [0, 0],
            colour: RGBA::default(),
            image: Image::default(),
            radius: Radius::default(),
            borders: None,
        }
    }
}

impl Widget for ImageButton {
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
