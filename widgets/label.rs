// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann_dev::Widget;

use crate::{
    graphics::colours::RGBA,
    theme::TextTheme,
    ToAny,
};

use super::{DebugWidget, Widget};

/// Label widget, which is not a "surfaced" widget. The text has to be rendered,
/// following its text theme.
///
/// Check <https://github.com/mooman219/fontdue> to render text with font.
#[derive(Debug, Clone, PartialEq, Widget)]
pub struct Label {
    /// The text string of the label.
    pub text: String,
    /// The theme for the text.
    pub theme: TextTheme,
}

/// Creates a new label. When there is only the text specified, it is not 
/// necessary to name the parameter but it must be a string literal.
#[macro_export]
macro_rules! label {
    ($text:literal $(,)?) => {
        Label::simple($text)
    };

    (text: $text:expr $(,)?) => {
        Label::simple($text),
    };

    (text: $text:expr, theme: $theme:expr $(,)?) => {
        Label::new($text, $theme),
    };
}

impl Widget for Label {
    fn build(&self) -> Box<dyn Widget> {
        self.clone().into()
    }

    fn colour(&self) -> RGBA {
        self.theme.colour
    }
}

impl Default for Label {
    fn default() -> Self {
        Self {
            text: String::new(),
            theme: TextTheme::default(),
        }
    }
}

impl Label {
    /// Creates a label with a specific `theme`.
    pub fn new(text: &str, theme: TextTheme) -> Self {
        Self {
            text: text.to_string(),
            theme,
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
