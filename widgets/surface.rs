// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann_dev::Widget;

use crate::{
    DebugWidget, 
    ToAny, 
    Widget, themes::{Theme, Style}, 
};

/// A drawable zone which can be coloured, bordered or both.
#[derive(Debug, Clone, Widget)]
pub struct Surface {
    /// Independent style for the surface.
    /// 
    /// If set as `None`, the default widget style from the global theme will 
    /// be used.
    style: Option<Style>,
}

impl Widget for Surface {
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

impl Surface {
    /// Creates a new surface with a colour and borders.
    pub fn new(style: Option<Style>) -> Self {
        Self { style }
    }

    /// Creates a new surface without independent style.
    pub fn simple() -> Self {
        Self { style: None }
    }
}
