// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//
// TODO: Absolutely everything but it had to be created because of the 
// drawables.
#![allow(missing_docs)]
//
//

use haussmann_dev::Widget;

use crate::{
    graphics::colours::RGBA,
    Border, DebugWidget, ToAny, Widget,
};

/// Image widget.
#[derive(Debug, Clone, PartialEq, Widget)]
pub struct Image {
    pub colour: RGBA,
    pub borders: Option<[Border; 4]>,
    /// Aspect ratio of the image.
    pub ratio: (f32, f32),
}

impl Widget for Image {
    fn build(&self) -> Box<dyn Widget> {
        self.clone().into()
    }

    fn colour(&self) -> RGBA {
        self.colour
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            colour: RGBA::default(),
            borders: None,
            ratio: (1.0, 1.0),
        }
    }
}
