// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{theme::TextTheme, ToAny};
use super::{Widget, DebugWidget};

/// Label widget, which is not a "surfaced" widget. The text has to be rendered, 
/// following its text theme.
/// 
/// Check https://github.com/mooman219/fontdue to render text with font.
#[derive(Debug, Clone)]
pub struct Label {
    pub text: String,
    /// Independent text theme for this label.
    /// 
    /// If the theme is `None`, the global theme for texts is used.
    pub theme: Option<TextTheme>,
}

crate::dynamic_widget!(Label);

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            theme: None,
        }
    }
}

impl Widget for Label {

}
