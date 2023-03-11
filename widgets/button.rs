// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann_dev::Widget;

use crate::{
    widgets,
    Align, Direction, Overflow, ToAny, themes::{Style, Theme},
};

use super::{DebugWidget, Label, Layout, Widget};

/// Button widget with a label inside.
#[derive(Debug, Clone, Widget)]
pub struct Button {
    /// Independent style for the button.
    /// 
    /// If set as `None`, the default button style from the global theme will 
    /// be used.
    pub style: Option<Style>,
    /// The label in the center of the button.
    pub label: Label,
}

/// Creates a new [`Button`] wrapped in a 
/// [`tap::Detector`](crate::controllers::tap::Detector).
#[macro_export]
macro_rules! button {
    ($label:expr, on_tap: |$button:ident, $theme:ident| $on_tap:block $(,)?) => {
        tap::Detector::new(Button::normal($label), |$button| $on_tap)
    };

    ($style:expr, $label:expr, on_tap: |$button:ident, $theme:ident| $on_tap:block $(,)?) => {
        tap::Detector::new(Button::styled($style, $label), |$button, $theme| $on_tap)
    };
}

impl Widget for Button {
    fn build(&self) -> Box<dyn Widget> {
        Layout {
            style: self.style.clone(),
            overflow: Overflow::Hide,
            wx_align: Align::Center,
            wy_align: Align::Center,
            direction: Direction::Column,
            widgets: widgets![self.label.clone()],
        }
        .into()
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

impl Button {
    /// Creates a new button with an independent style.
    pub fn styled(style: Style, label: Label) -> Self {
        Self {
            style: Some(style),
            label,
        }
    }

    /// Creates a button without independent style.
    pub fn normal(label: Label) -> Self {
        Self {
            style: None,
            label,
        }
    }
}
