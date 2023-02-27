// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    graphics::{Shape, Size, Point},
    DebugWidget, ToAny, Widget,
};

/// Image widget.
#[derive(Debug, Clone)]
pub struct Image {
    /// Aspect ratio of the image.
    pub ratio: (f32, f32),
    // todo
}

crate::dynamic_widget!(Image);

impl Widget for Image {
    fn shape(&self, _position: Option<Point>, _size: Option<Size>) -> Shape {
        panic!("cannot return shape")
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            ratio: (1.0, 1.0),
            // todo
        }
    }
}

impl Image {
    /// Creates a new image.
    pub fn new(ratio: (f32, f32)) -> Self {
        Self {
            ratio,
            // todo
        }
    }
}
