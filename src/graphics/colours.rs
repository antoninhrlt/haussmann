// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

pub const TRANSPARENT: RGBA = RGBA { r: 0, g: 0, b: 0, a: 0 };
pub const RED: RGBA = RGBA { r: 255, g: 0, b: 0, a: 255 };
pub const GREEN: RGBA = RGBA { r: 0, g: 255, b: 0, a: 255 };
pub const BLUE: RGBA = RGBA { r: 0, g: 0, b: 255, a: 255 };

/// Colour with red, green and blue values + an alpha channel.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RGBA {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

impl Default for RGBA {
    fn default() -> Self {
        RGBA { r: 0, g: 0, b: 0, a: 0 }
    }
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
        (self.a << 24) | (self.b << 16) | (self.g << 8) | self.r
    }
}
