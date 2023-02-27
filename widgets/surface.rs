// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{Widget, ToAny, DebugWidget, graphics::{colours::RGBA, Size, Shape, shapes, Point}, Border, widgets};

#[derive(Debug)]
pub struct Surface {
    colour: RGBA,
    borders: Option<[Border; 4]>,
}

widgets::dynamic_widget!(Surface);

impl Widget for Surface {
    fn shape(&self, position: Option<Point>, size: Option<Size>) -> Shape {
        assert_ne!(position, None);
        assert_ne!(size, None);

        shapes::Builder::new()
            .rectangle_at(position.unwrap(), size.unwrap(), self.borders)
            .fill(self.colour)
            .finish()
    }
}

impl Surface {
    pub fn new(colour: RGBA, borders: [Border; 4]) -> Self {
        Self {
            colour,
            borders: Some(borders),
        }
    }

    pub fn coloured(colour: RGBA) -> Self {
        Self {
            colour,
            borders: None,
        }
    }

    pub fn bordered(borders: [Border; 4]) -> Self {
        Self {
            colour: RGBA::default(),
            borders: Some(borders),
        }
    }
}
