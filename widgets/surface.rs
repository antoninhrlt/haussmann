// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{Widget, ToAny, DebugWidget, graphics::{colours::RGBA, Size, Shape, shapes, Point}, Border, widgets};

#[derive(Debug, Clone)]
pub struct Surface {
    pub colour: RGBA,
    pub borders: Option<[Border; 4]>,
}

widgets::dynamic_widget!(Surface);

impl Widget for Surface {
    fn build(&self) -> Box<dyn Widget> {
        self.clone().into()
    }

    fn colour(&self) -> RGBA {
        self.colour
    }
}

impl Surface {
    pub(crate) fn to_shape(&self, position: Point, size: Size) -> Shape {
        shapes::Builder::new()
            .rectangle_at(position, size, self.borders)
            .fill(self.colour)
            .finish()
    }
}

impl Surface {
    pub fn new(colour: RGBA, borders: Option<[Border; 4]>) -> Self {
        Self {
            colour,
            borders,
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
