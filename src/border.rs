// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::graphics::colours::RGBA;

/// Rectangle border of `width`, of `colour`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Border {
    width: i32,
    colour: RGBA,
}

impl Border {
    pub fn new(width: i32, colour: RGBA) -> Self {
        Self {
            width,
            colour,
        }
    }
}
