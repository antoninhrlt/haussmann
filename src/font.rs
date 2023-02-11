// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use std::collections::HashMap;

/// Font family with a `name` for the whole family.
/// 
/// Each font from `fonts` is the font associated to each font's weight.
#[derive(Debug, Clone)]
pub struct FontFamily {
    pub name: String,
    pub fonts: HashMap<FontWeight, TTFFont>
}

/// TTF Font with a `name`.
#[derive(Debug, Clone)]
pub struct TTFFont {
    /// The font's name identifier.
    pub name: String,
    /// The path of the TTF file.
    pub path: String,
}

/// Font weights
#[derive(Debug, Clone)]
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
