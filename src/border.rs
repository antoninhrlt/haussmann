// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::graphics::colours::RGBA;

/// Rectangle border of width a width and a colour.
///
/// Borders are more commonly use in arrays like `[Border; 4]` to define a
/// border on each side of a shape.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Border {
    /// The width of the border.
    pub width: i32,
    /// The colour of the border.
    pub colour: RGBA,
}

impl Border {
    /// Creates a new sized and coloured border.
    pub fn new(width: i32, colour: RGBA) -> Self {
        Self { width, colour }
    }
}
