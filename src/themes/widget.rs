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

/// Creates ""dynamically"" a new [`Style`].
#[macro_export]
macro_rules! style {
    (colour: $colour:expr, borders: $borders:expr, radius: $radius:expr $(,)?) => {
        Style {
            colour: Some($colour),
            borders: Some($borders),
            radius: Some($radius),
        }
    };

    (colour: $colour:expr, borders: $borders:expr $(,)?) => {
        Style {
            colour: Some($colour),
            borders: Some($borders),
            radius: None,
        }
    };

    (colour: $colour:expr, radius: $radius:expr $(,)?) => {
        Style {
            colour: Some($colour),
            borders: None,
            radius: Some($radius),
        }
    };

    (borders: $borders:expr, radius: $radius:expr $(,)?) => {
        Style {
            colour: $colour,
            borders: Some($borders),
            radius: Some($radius),
        }
    };

    (colour: $colour:expr $(,)?) => {
        Style {
            colour: Some($colour),
            borders: None,
            radius: None,
        }
    };

    (borders: $borders:expr $(,)?) => {
        Style {
            colour: None,
            borders: Some($borders),
            radius: None,
        }
    };

    (radius: $radius:expr $(,)?) => {
        Style {
            colour: None,
            borders: None,
            radius: Some($radius),
        }
    };
}
