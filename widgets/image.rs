// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    graphics::{Shape, Size, Point, colours::RGBA},
    DebugWidget, ToAny, Widget, widgets, Border,
};

use super::Surface;

/// Image widget.
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub colour: RGBA,
    pub borders: Option<[Border; 4]>,
    /// Aspect ratio of the image.
    pub ratio: (f32, f32),
}

widgets::dynamic_widget!(Image);

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
