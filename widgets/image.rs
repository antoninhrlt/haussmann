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

use crate::{ DebugWidget, ToAny, Widget, themes::{Style, Theme} };

/// Image widget.
#[derive(Debug, Clone, PartialEq, Widget)]
pub struct Image {
    /// Independent style for the image.
    /// 
    /// If set as `None`, the default widget style of the global theme will be 
    /// used. 
    pub style: Option<Style>,
    /// Aspect ratio of the image.
    pub ratio: (f32, f32),
}

impl Widget for Image {
    fn build(&self) -> Box<dyn Widget> {
        self.clone().into()
    }

    fn style(&self, theme: &Theme) -> Style {
        match &self.style {
            Some(style) => style,
            None => &theme.style
        }
        .clone()
    }
    
    fn style_mut(&mut self, theme: &Theme) -> &mut Style {
        if let None = self.style {
            self.style = Some(theme.style.clone()); 
        }

        self.style.as_mut().unwrap()
    }
}
