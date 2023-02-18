// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use std::f32::consts;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Radius(f32);

impl Default for Radius {
    fn default() -> Self {
        Radius(0.0)
    }
}

impl Radius {
    pub fn new(degrees: f32) -> Self {
        Self(degrees)
    }

    /// Converts the `value` given in radians to degrees.
    pub fn from_radians(value: f32) -> Self {
        Self(value * consts::PI / 180.0)
    }
}
