// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    FontFamily,
};

use super::TextTheme;

#[derive(Debug)]
pub struct ViewTheme {
    /// All the font families used in the view.
    /// 
    /// ## Help
    /// Retrieve a font by its name by calling [`font`](ViewTheme::font).
    pub fonts: Vec<FontFamily>,
    /// Theme for the texts.
    pub text_theme: TextTheme,
}

impl ViewTheme {
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
