// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    graphics::{ colours::RGBA, Point, Size, }, 
    Zone, 
    Side
};

/// Rectangle border of width a width and a colour.
///
/// Borders are more commonly use in arrays like `[Border; 4]` to define a
/// border on each side of a shape.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Border {
    /// The width of the border.
    pub width: usize,
    /// The colour of the border.
    pub colour: RGBA,
    /// On which side is this border.
    pub side: Side,
}

impl Border {
    /// Creates a new sized and coloured border.
    pub fn new(width: usize, colour: RGBA, side: Side) -> Self {
        Self { width, colour, side }
    }

    /// Creates a zone of the border's size and width or height of the parent.
    pub fn as_zone(&self, parent_zone: &Zone) -> Zone {
        Zone {
            position: Point::from(match self.side {
                Side::Left => [parent_zone.x(), parent_zone.y()],
                Side::Right => [parent_zone.x() + parent_zone.width() as isize, parent_zone.y()],
                Side::Bottom => [parent_zone.x(), parent_zone.y() + parent_zone.height() as isize],
                Side::Top => [parent_zone.x(), parent_zone.y()],
            }),
            size: Size::from(match self.side {
                Side::Left | Side::Right => [self.width, parent_zone.height()],
                Side::Top | Side::Bottom => [parent_zone.width(), self.width],
            }),
        }
    }
}
