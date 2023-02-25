// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{dynamic_widget, Widget, ToAny, DebugWidget, graphics::{colours::RGBA, Size, Shape, shapes}, Border};

#[derive(Debug)]
pub struct Surface {
    colour: RGBA,
    borders: Option<[Border; 4]>,
}

dynamic_widget!(Surface);

impl Widget for Surface {
    fn shape(&self, size: Option<Size>) -> Shape {
        assert!(size != None);

        shapes::Builder::new()
            .rectangle(size.unwrap(), self.borders)
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
