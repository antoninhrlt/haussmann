// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

/// Colour with red, green and blue values + an alpha channel.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RGBA {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

impl RGBA {
    /// Creates a new `RGBA` object.
    /// 
    /// `r`, `g`, `b` and `a` are in range 0 to 255, where 0 is the non-colour 
    /// (black) and 255 the full colour (red, green or blue). For `a` being the 
    /// alpha channel, 0 transparent and 255 is completely visible.
    pub fn new(r: u32, g: u32, b: u32, a: u32) -> Self {
        Self { r, g, b, a, }
    }

    /// Creates a new `RGBA` object from an hexadecimal value having an alpha 
    /// channel.
    /// 
    /// Converts the hexadecimal `value` into a "RGBA" structure.
    pub fn from_hex(value: u32) -> Self {
        Self {
            r: ((value >> 24) & 0xFF) / 255,    // RR byte
            g: ((value >> 16) & 0xFF) / 255,    // GG byte
            b: ((value >> 8) & 0xFF) / 255,     // BB byte
            a: ((value) & 0xFF) / 255,          // AA byte
        }
    }

    /// Converts the `RGBA` values into an hexadecimal value.
    pub fn to_hex(&self) -> u32 {
        ((self.r & 0xFF) << 24) 
            + ((self.g & 0xFF) << 16) 
            + ((self.b & 0xFF) << 8)
            + (self.a & 0xFF)
    }
}
