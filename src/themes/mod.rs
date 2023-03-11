// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything about theming and styling.
//! 
//! ## Important notes
//! It is possible to see two different types of themes: "styles" and "themes".
//! - A theme is a collection of styles. Usually, each style has an associated 
//! name.
//! - A style defines the graphical properties of an item such as the colour, 
//! the size, etc...

mod text;
mod widget;

pub use text::*;
pub use widget::*;

use crate::{FontFamily, FontWeight, graphics::colours::RGBA, Radius};

/// What is called a "global theme" in the whole project. Contains fonts, text 
/// theme and styles. 
#[derive(Debug)]
pub struct Theme {
    /// All the font families used in the view.
    /// 
    /// ## Help
    /// Retrieve a font by its name by calling [`font`](ViewTheme::font).
    pub fonts: Vec<FontFamily>,
    /// Theme for the texts.
    pub text_theme: TextTheme,
    /// Fallback style for labels.
    pub label_style: LabelStyle,
    /// Fallback style for widgets.
    pub style: Style,
}

impl Theme {
    /// Returns the font with the same name if exists.
    pub fn font(&self, name: String) -> Option<FontFamily> {
        // Browses all the fonts to find a font with the same name.
        for font in &self.fonts {
            // The font has the same name, returns it.
            if font.name == name {
                return Some(font.clone());
            }
        }

        // Not font with the same name has been found.
        None
    }
}

/// Returns the default project's theme.
/// 
/// At least one font family must be given.
pub fn default(fonts: Vec<FontFamily>) -> Theme {
    Theme { 
        fonts,
        text_theme: TextTheme {
            code: TextStyle {
                size: 12,
                weight: FontWeight::Regular,
                spacing: 0.25,
            },
            heading1: TextStyle {
                size: 57,
                weight: FontWeight::Regular,
                spacing: 0.0,
            },
            heading2: TextStyle {
                size: 45,
                weight: FontWeight::Regular,
                spacing: 0.0,
            },
            heading3: TextStyle {
                size: 36,
                weight: FontWeight::Regular,
                spacing: 0.0,
            },
            heading4: TextStyle {
                size: 32,
                weight: FontWeight::Regular,
                spacing: 0.0,
            },
            heading5: TextStyle {
                size: 28,
                weight: FontWeight::Regular,
                spacing: 0.0,
            },
            heading6: TextStyle {
                size: 24,
                weight: FontWeight::Regular,
                spacing: 0.0,
            },
            paragraph1: TextStyle {
                size: 16,
                weight: FontWeight::Regular,
                spacing: 0.15,
            },
            paragraph2: TextStyle {
                size: 14,
                weight: FontWeight::Regular,
                spacing: 0.25,
            },
            paragraph3: TextStyle {
                size: 12,
                weight: FontWeight::Regular,
                spacing: 0.40,
            },
        },
        label_style: LabelStyle { 
            colour: RGBA::new(0, 0, 0, 255),
            text_style: None,
        },
        style: Style { 
            colour: Some(RGBA::new(180, 180, 180, 255)),
            borders: Some([None, None, None, None]),
            radius: Some(Radius::default()),
        },
    }
}
