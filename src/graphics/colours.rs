// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Structures to store and convert hexadecimal to RGBA colour values, and
//! colour constants.

/// Blue colour constant (alpha = 255).
pub const BLUE: RGBA = RGBA {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};
/// Green colour constant (alpha = 255).
pub const GREEN: RGBA = RGBA {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};
/// Red colour constant (alpha = 255).
pub const RED: RGBA = RGBA {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
/// Transparent colour constant (alpha = 0).
pub const TRANSPARENT: RGBA = RGBA {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

/// Colour with red, green and blue values + an alpha channel.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct RGBA {
    /// Red part of the colour corresponding to the first two bytes of an
    /// hexadecimal colour value.
    ///
    /// Must be in range from 0 to 255.
    pub r: u32,
    /// Green part of the colour corresponding to the third and fourth bytes of
    /// an hexadecimal colour value.
    ///
    /// Must be in range from 0 to 255.
    pub g: u32,
    /// Blue part of the colour corresponding to the fifth and sixth bytes of an
    /// hexadecimal colour value.
    ///
    /// Must be in range from 0 to 255.
    pub b: u32,
    /// Alpha part of the colour corresponding to the seventh and eighth bytes
    /// of an hexadecimal colour value.
    ///
    /// Must be in range from 0 to 255.
    pub a: u32,
}

impl RGBA {
    /// Creates a new `RGBA` object.
    ///
    /// `r`, `g`, `b` and `a` are in range 0 to 255, where 0 is the non-colour
    /// (black) and 255 the full colour (red, green or blue). For `a` being the
    /// alpha channel, 0 transparent and 255 is completely visible.
    pub fn new(r: u32, g: u32, b: u32, a: u32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a new `RGBA` object from an hexadecimal value having an alpha
    /// channel.
    ///
    /// Converts the hexadecimal `value` into a "RGBA" structure.
    pub fn from_hex(value: u32) -> Self {
        Self {
            r: ((value >> 24) & 0xFF) / 255, // RR byte
            g: ((value >> 16) & 0xFF) / 255, // GG byte
            b: ((value >> 8) & 0xFF) / 255,  // BB byte
            a: ((value) & 0xFF) / 255,       // AA byte
        }
    }

    /// Converts the `RGBA` values into an hexadecimal value.
    pub fn to_hex(&self) -> u32 {
        (self.a << 24) | (self.b << 16) | (self.g << 8) | self.r
    }
}

/// Creates a new [`RGBA`] object. The alpha channel is explicitly written with 
/// `a:`.
#[macro_export]
macro_rules! rgba {
    ($r:expr, $g:expr, $b:expr, a: $a:expr $(,)*) => {
        RGBA::new($r, $g, $b, $a)
    };
}

/// Converts an hexadecimal colour value into a [`RGBA`] object.
#[macro_export]
macro_rules! hex {
    ($colour:expr) => {
        RGBA::from_hex($colour)
    };
}
