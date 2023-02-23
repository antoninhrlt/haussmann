// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{theme::TextTheme, ToAny, graphics::{Size, Shape}};
use super::{Widget, DebugWidget};

/// Label widget, which is not a "surfaced" widget. The text has to be rendered, 
/// following its text theme.
/// 
/// Check https://github.com/mooman219/fontdue to render text with font.
#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub text: String,
    /// Independent text theme for this label.
    /// 
    /// If the theme is `None`, the global theme for texts is used.
    pub theme: Option<TextTheme>,
}

crate::dynamic_widget!(Label);

impl Default for Label {
    fn default() -> Self {
        Self {
            text: String::new(),
            theme: None,
        }
    }
}

impl Label {
    /// Creates a label with a specific `theme`.
    pub fn new(text: &str, theme: TextTheme) -> Self {
        Self {
            text: text.to_string(),
            theme: Some(theme),
        }
    }

    /// Creates a label without specific `theme`, the global theme for texts 
    /// is used.
    pub fn simple(text: &str) -> Self {
        Self {
            text: text.to_string(),
            ..Self::default()
        }
    }
}

impl Widget for Label {
    fn shape(&self, size: Size) -> Shape {
        panic!("cannot return shape")
    }
}
