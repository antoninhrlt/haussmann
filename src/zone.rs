// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::graphics::{Point, Size};

/// Sized and positioned rectangle.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Zone {
    /// The top-left position of the zone.
    pub position: Point,
    /// the size of the zone.
    pub size: Size,
}

impl From<(Point, Size)> for Zone {
    fn from(value: (Point, Size)) -> Self {
        Zone {
            position: value.0,
            size: value.1,
        }
    }
}

impl Zone {
    /// Returns the x position of the zone.
    pub fn x(self) -> isize {
        self.position[0]
    }

    /// Returns the y position of the zone.
    pub fn y(self) -> isize {
        self.position[1]
    }

    /// returns the width of the zone.
    pub fn width(self) -> usize {
        self.size[0]
    }

    /// Returns the height of the zone.
    pub fn height(self) -> usize {
        self.size[1]
    }
}
