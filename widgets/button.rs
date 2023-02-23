// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{graphics::{Shape, shapes, colours::RGBA, Size}, Border, Radius, ToAny};
use super::{Widget, DebugWidget};

#[derive(Debug, Clone)]
pub struct Button {
    pub colour: RGBA,
    pub radius: Radius,
    pub borders: Option<[Border; 4]>,
}

crate::dynamic_widget!(Button);

impl Widget for Button {
    fn shape(&self, size: Size) -> Shape {
        shapes::Builder::new()
            .rectangle(size, self.borders)
            .fill(self.colour)
            .finish()
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            colour: RGBA::new(0, 0, 0, 0),
            radius: Radius::new(0.0),
            borders: None,
        }
    }
}

impl Button {
    /// Creates a button with a `radius` and `borders`.
    pub fn new(colour: RGBA, radius: Radius, borders: [Border; 4]) -> Self {
        Self {
            colour,
            radius,
            borders: Some(borders),
        }
    }

    /// Creates the simplest button possible, without radius nor borders.
    pub fn simple(colour: RGBA) -> Self {
        Self {
            colour,
            ..Self::default()
        }
    }

    /// Creates a button with a `radius` but no `borders`.
    pub fn rounded(size: Size, colour: RGBA, radius: Radius) -> Self {
        Self {
            colour,
            radius,
            ..Self::default()
        }
    }

    /// Creates a button with `borders` but no `radius`.
    pub fn bordered(size: Size, colour: RGBA, borders: [Border; 4]) -> Self {
        Self {
            colour,
            borders: Some(borders),
            ..Self::default()
        }
    }
}
