// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{graphics::colours::RGBA, Border, Radius};

use super::text::TextStyle;

/// Style for a label.
#[derive(Debug, Clone, PartialEq)]
pub struct LabelStyle {
    /// The colour of the text.
    pub colour: RGBA,
    /// Independent Style for the text.
    /// 
    /// If set as `None`, the default text style from the global theme will be 
    /// used.
    pub text_style: Option<TextStyle>,
}

/// Style for any widget.
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// Independent colour from the global theme.
    /// 
    /// If set as `None`, use the colour of the default widget style from the 
    /// global theme.
    pub colour: Option<RGBA>,
    /// Independent borders from the global theme.
    /// 
    /// If set as `None`, use the borders of the default widget style from the 
    /// global theme.
    pub borders: Option<[Option<Border>; 4]>,
    /// Independent radius from the global theme.
    /// 
    /// If set as `None`, use the radius of the default widget style from the 
    /// global theme.
    pub radius: Option<Radius>,
}

impl Default for Style {
    fn default() -> Self {
        Style { colour: None, borders: None, radius: None }
    }
}
