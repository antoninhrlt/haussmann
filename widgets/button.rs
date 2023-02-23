// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{graphics::{Shape, shapes, colours::RGBA, Size, self}, Border, Radius, ToAny};
use super::{Widget, DebugWidget, Label};

#[derive(Debug, Clone)]
pub struct Button {
    pub label: Label,
    pub colour: RGBA,
    pub radius: Radius,
    pub borders: Option<[Border; 4]>,
}

crate::dynamic_widget!(Button);

impl Widget for Button {
    fn shape(&self, size: Option<Size>) -> Shape {
        shapes::Builder::new()
            .rectangle(size.unwrap(), self.borders)
            .fill(self.colour)
            .finish()
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            label: Label::default(),
            colour: RGBA::new(0, 0, 0, 0),
            radius: Radius::new(0.0),
            borders: None,
        }
    }
}

impl Button {
    pub fn new(label: Label, colour: RGBA, radius: Radius, borders: [Border; 4]) -> Self {
        Self {
            label,
            colour,
            radius,
            borders: Some(borders),
        }
    }

    /// Creates the simplest button possible, without radius nor borders.
    pub fn simple(label: Label, colour: RGBA) -> Self {
        Self {
            label,
            colour,
            ..Self::default()
        }
    }

    /// Creates a button with a `radius` but no borders.
    pub fn rounded(label: Label, colour: RGBA, radius: Radius) -> Self {
        Self {
            label,
            colour,
            radius,
            ..Self::default()
        }
    }

    /// Creates a button with `borders` but no radius.
    pub fn bordered(label: Label, colour: RGBA, borders: [Border; 4]) -> Self {
        Self {
            label,
            colour,
            borders: Some(borders),
            ..Self::default()
        }
    }
}
