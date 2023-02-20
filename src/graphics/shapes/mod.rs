// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{Border, Radius};
use super::Point;
use super::colours::RGBA;

mod builder;
pub mod rectangle;

/// `Builder` is accessible from `crate::graphics::shapes::Builder`.
pub use builder::Builder;

/// The parent of any sort of shape. To create a `Shape` object, see 
/// `ShapeBuilder`.
/// 
/// There is the same number of `points` than `borders`. The shape cannot be set 
/// as `filled` without fill colour.
/// 
/// Thanks to `ShapeBuilder` it's surely a safe structure, there cannot be any 
/// illogical problem.
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    /// The points of the shape.
    /// 
    /// The first point is the top left point. It is also relative to the layout
    /// where the shape is, it is not a fixed point.
    points: Vec<Point>,
    /// Borders for each side of the shape. An empty `borders` vector is used to 
    /// define no borders.
    borders: Vec<Border>,
    /// The first value is whether the shape is filled, and the second value is
    /// the fill colour if the shape is filled.
    /// 
    /// If the shape is designed to be filled, the second value must be not 
    /// `None`.
    filled: (bool, Option<RGBA>),
    /// The radius of the shape, in degrees.
    radius: Radius,
}

impl Shape {
    /// Moves all the points by `position`.
    pub fn move_by(&mut self, position: Point) {
        for point in &mut self.points {
            *point = [position[0] + point[0], position[1] + point[1]];
        }
    }

    /// Returns the shape's points.
    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }
    
    /// Returns the first point of the shape being its position.
    pub fn position(&self) -> &Point {
        &self.points[0]
    }

    /// Returns the shape's borders if they exist, otherwise returns `None`. 
    pub fn borders(&self) -> Option<&Vec<Border>> {
        if self.borders.is_empty() {
            None
        } else {
            Some(&self.borders)
        }
    }

    /// Whether the shape is filled or not.
    pub fn is_filled(&self) -> bool {
        self.filled.0
    }

    /// Returns the fill colour when exists, otherwise returns `None`. But, also
    /// returns `None` if the shape is specified to not be filled, no matter if 
    /// the fill colour exists or not.
    pub fn fill_colour(&self) -> Option<RGBA> {
        if !self.is_filled() {
            // The shape is not filled, there is no fill colour or we don't care 
            // about it.
            None
        } else {
            // Returns the fill colour
            self.filled.1
        }
    }

    /// Returns the radius for the shape's corners.
    pub fn radius(&self) -> Radius {
        self.radius
    }
}
