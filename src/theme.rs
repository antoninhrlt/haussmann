// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{graphics::colours::RGBA, font::FontWeight, FontFamily, TextAlign, Align};

/// Theme for a text with a font.
#[derive(Debug, Clone, PartialEq)]
pub struct TextTheme {
    /// The height of the text.
    pub font_size: u32,
    /// The weight of the letters.
    pub font_weight: FontWeight,
    /// The colour of the text.
    pub colour: RGBA,
    /// Text alignment.
    pub align: TextAlign,
}

impl Default for TextTheme {
    fn default() -> Self {
        Self {
            font_size: 16,
            font_weight: FontWeight::default(),
            colour: RGBA::default(),
            align: Align::Left,
        }
    }
}

/// Global search to be used as a fallback theme when a widget has a theme set 
/// as `None`. 
/// Themes for different widgets or objects.
#[derive(Debug, Clone)]
pub struct Theme {
    /// The main colour.
    pub primary_colour: RGBA,
    /// The secondary main colour.
    pub secondary_colour: RGBA,
    /// Colour used for highlighting.
    pub highlight_colour: RGBA,
    /// Colour used on focusing.
    pub focus_colour: RGBA,
    /// Font family for all the texts.
    pub font: Option<FontFamily>,
    /// Text theme for titles.
    pub title: TextTheme,
    /// Text theme for subtitles.
    pub subtitle: TextTheme,
    /// Text theme for normal paragraph texts.
    pub text1: TextTheme,
    /// Another text theme for other normal paragraph texts.
    pub text2: TextTheme,
}

impl Default for Theme {
    /// The default theme for any application using this crate. It's an 
    /// high-contrast theme, with colourful accent colours, not very pretty.
    fn default() -> Self {
        Self {
            primary_colour: RGBA::new(255, 255, 255, 255),
            secondary_colour: RGBA::new(0, 0, 0, 255),
            highlight_colour: RGBA::new(255, 0, 0, 255),
            focus_colour: RGBA::new(0, 0, 0, 100),
            font: None,
            title: TextTheme { font_size: 24, font_weight: FontWeight::Bold, colour: RGBA::new(0, 0, 0, 255), align: TextAlign::Center },
            subtitle: TextTheme { font_size: 18, font_weight: FontWeight::SemiBold, colour: RGBA::new(0, 0, 0, 255), align: TextAlign::Center },
            text1: TextTheme { font_size: 12, font_weight: FontWeight::Medium, colour: RGBA::new(0, 0, 0, 255), align: TextAlign::Left },
            text2: TextTheme { font_size: 12, font_weight: FontWeight::Medium, colour: RGBA::new(0, 0, 0, 255), align: TextAlign::Left },
        }
    }
}

impl Theme {
    /// The ayu theme from <https://raw.githubusercontent.com/ayu-theme/ayu-colors/master/colors.svg>
    pub fn ayu() -> Self {
        let font_colour = RGBA::from_hex(0xBFBDB6);

        Self {
            primary_colour: RGBA::from_hex(0x0D101700),
            secondary_colour: RGBA::from_hex(0x131721),
            highlight_colour: RGBA::from_hex(0xE6B450),
            focus_colour: RGBA::from_hex(0x47526640),
            font: None,
            title: TextTheme { font_size: 24, font_weight: FontWeight::Bold, colour: font_colour, align: TextAlign::Center },
            subtitle: TextTheme { font_size: 18, font_weight: FontWeight::SemiBold, colour: font_colour, align: TextAlign::Center },
            text1: TextTheme { font_size: 12, font_weight: FontWeight::Medium, colour: font_colour, align: TextAlign::Left },
            text2: TextTheme { font_size: 12, font_weight: FontWeight::Medium, colour: RGBA::from_hex(0x39BAE6), align: TextAlign::Left },
        }
    }
}
