// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use std::collections::HashMap;

/// Local path for a font file, to be associated to a name.
#[derive(Debug, Clone, PartialEq)]
pub struct Font {
    /// Name for the font. Used to identify it.
    pub name: String,
    /// Local path to the font file.
    pub path: String,
}

/// Named collection of [`Font`s](Font) where each font is associated to a 
/// [`FontWeight`].
#[derive(Debug, Clone)]
pub struct FontFamily {
    /// Name for the font family. Used to identify it.
    pub name: String,
    /// Font for each available [`FontWeight`].
    pub fonts: HashMap<FontWeight, Font>,
}

/// Font weight to be associated to a [`Font`] in a [`FontFamily`].
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
