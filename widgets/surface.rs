// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann_dev::Widget;

use crate::{
    graphics::colours::RGBA, 
    Widget, 
    ToAny, 
    DebugWidget, 
    Border, 
};

#[derive(Debug, Clone, Widget)]
pub struct Surface {
    pub colour: RGBA,
    pub borders: Option<[Border; 4]>,
}

/// Creates a new surface, either transparent, with colour or borders or both.
#[macro_export]
macro_rules! surface {
    () => {
        Surface::default()
    };
    
    (colour: $colour:expr $(,)*) => {
        Surface::coloured($colour)
    };

    (borders: $borders:expr $(,)*) => {
        Surface::bordered($borders)
    };

    (colour: $colour:expr, borders: $borders:expr $(,)*) => {
        Surface::new($colour, Some($borders))
    };
}

impl Widget for Surface {
    fn build(&self) -> Box<dyn Widget> {
        self.clone().into()
    }

    fn colour(&self) -> RGBA {
        self.colour
    }
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            colour: RGBA::default(),
            borders: None,
        }
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
