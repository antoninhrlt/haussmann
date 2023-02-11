// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

/// Font structured with a name, a size and a TTF file path associated.
/// 
/// For bold, italic or any other "kind" of fonts, simply create another `Font` 
/// object.
pub struct Font {
    /// The font's name identifier.
    pub name: String,
    /// The path of the TTF file.
    pub path: String,
    /// The font's size.
    pub size: f32,
}
