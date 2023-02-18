// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{graphics::{Shape, shapes, colours::RGBA, Size}, Border, Radius, ToAny};
use super::{Widget, CreateWidget, DebugWidget, Label};

/// Button widget, without text.
#[derive(Debug, Clone)]
pub struct Button {
    size: Size,
    colour: RGBA,
    pub radius: Radius,
    pub borders: Option<[Border; 4]>,
}

crate::dynamic_widget!(Button);

impl Button {
    /// Creates the most complex button possible. 
    /// 
    /// However, `borders` can be `None`.
    fn all(size: Size, colour: RGBA, radius: Option<Radius>, borders: Option<[Border; 4]>) -> Self {
        Self {
            size,
            colour,
            radius: radius.unwrap_or(Radius::new(0.0)),
            borders,
        }
    }
}

impl CreateWidget for Button {
    /// Creates the simplest button possible.
    fn new(size: Size, colour: RGBA) -> Self {
        Self::all(size, colour, None, None)
    }
    
    /// Creates a button with a radius.
    fn rounded(size: Size, colour: RGBA, radius: Radius) -> Self {
        Self::all(size, colour, Some(radius), None)
    }

    /// Creates a button with borders.
    fn bordered(size: Size, colour: RGBA, borders: [Border; 4]) -> Self {
        Self::all(size, colour, None, Some(borders))
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
