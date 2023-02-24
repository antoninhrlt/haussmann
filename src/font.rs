// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use std::collections::HashMap;

/// Font family with a name for the whole family.
/// 
/// Each font from `fonts` is the font associated to each font's weight.
#[derive(Debug, Clone)]
pub struct FontFamily {
    /// The name of the font family.
    pub name: String,
    /// Font for each available [`FontWeight`].
    pub fonts: HashMap<FontWeight, TTFFont>
}

/// Named local TTF font.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TTFFont {
    /// The font's name identifier.
    pub name: String,
    /// The path of the TTF file.
    pub path: String,
}

/// The weight for a font.
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FontWeight {
    Black = 900,
    ExtraBold = 800,
    Bold = 700,
    SemiBold = 600,
    Medium = 500,
    Regular = 400,
    Light = 300,
    ExtraLight = 200,
    Thin = 100,
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Regular
    }
}