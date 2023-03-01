// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to drawing of widgets.

use crate::{widgets::{Label, Image, Surface}, Widget};

use super::{Size, Point};

/// Something that can be drawn. Each element has a defined position and size 
/// calculated at generation.
#[derive(Debug)]
pub enum Drawable {
    /// Image to draw at a defined position with a defined size.
    Image(Image, Point, Size),
    /// Label to draw at a defined position with a defined size.
    Label(Label, Point, Size),
    /// Surface to draw at a defined position with a defined size.
    Surface(Surface, Point, Size),
    /// Unknown widget to draw at a defined position with a defined size.
    Unknown(Box<dyn Widget>, Point, Size),
}

impl Drawable {
    /// Extracts the position and size of the drawable for the drawable itself.
    pub fn extract(&self) -> (&Point, &Size) {
        match self {
            Self::Image(_, p, s) => (p, s),
            Self::Label(_, p, s) => (p, s),
            Self::Surface(_, p, s) => (p, s),
            Self::Unknown(_, p, s) => (p, s),
        }
    }
}
