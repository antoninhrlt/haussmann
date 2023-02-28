// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    graphics::{
        colours::RGBA, 
        Size, Point
    }, 
    Widget, 
    ToAny, 
    DebugWidget, 
    Border, 
    widgets
};

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
