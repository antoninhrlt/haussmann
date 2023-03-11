// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann_dev::Widget;

use crate::{
    themes::{Theme, Style, LabelStyle},
    ToAny,
};

use super::{DebugWidget, Widget};

/// Label widget, which is not a "surfaced" widget. The text has to be rendered,
/// following its text theme.
///
/// Check <https://github.com/mooman219/fontdue> to render text with font.
#[derive(Debug, Clone, PartialEq, Widget)]
pub struct Label {
    /// An independent style from the global theme.
    /// 
    /// If set as `None`, the style for labels defined in the global theme 
    /// will be used.
    pub style: Option<LabelStyle>,
    /// The text string of the label.
    pub text: String,
}

impl Widget for Label {
    fn build(&self) -> Box<dyn Widget> {
        self.clone().into()
    }

    fn style(&self, _: &Theme) -> Style {
        panic!("labels have special styles (TextStyle). check the `Label::label_style()` function");
    }

    fn style_mut(&mut self, _: &Theme) -> &mut Style {
        panic!("labels have special styles (TextStyle). check the `Label::label_style_mut()` function");
    }
}

impl Label {
    /// Creates a label with an independent style.
    pub fn styled(text: &str, style: LabelStyle) -> Self {
        Self {
            style: Some(style),
            text: text.to_string(),
        }
    }

    /// Creates a label without independent style.
    pub fn normal(text: &str) -> Self {
        Self {
            style: None,
            text: text.to_string(),
        }
    }

    /// Same as [`Widget::style()`] but labels return [`LabelStyle`] instead of
    /// normal [`Style`].
    /// 
    /// ## Note
    /// Calling [`Label::style`] panics. 
    pub fn label_style(&self, theme: &Theme) -> LabelStyle {
        match &self.style {
            Some(label_style) => label_style,
            None => &theme.label_style
        }
        .clone()
    }

    /// Same as [`Widget::style_mut()`] but labels return [`LabelStyle`] instead of
    /// normal [`Style`].
    /// 
    /// ## Note
    /// Calling [`Label::style_mut`] panics. 
    pub fn label_style_mut(&mut self, theme: &Theme) -> &mut LabelStyle {
        if let None = self.style {
            self.style = Some(theme.label_style.clone()); 
        }

        self.style.as_mut().unwrap()
    }
}
