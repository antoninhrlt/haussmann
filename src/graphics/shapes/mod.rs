// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use super::{ Size, Point };
use crate::Border;
use super::colours::RGBA;

/// The constant `N` stands for the number of points, and the number of borders 
/// of the shape. For example, `N = 4` is a rectangle.
#[derive(Debug)]
struct Shape<const N: usize> {
    /// The points of the shape.
    pub points: [Point; N],
    /// Borders for each side of the shape.
    borders: Option<[Border; N]>,
    /// The first value is whether the shape is filled, and the second value is
    /// the fill colour if the shape is filled.
    /// 
    /// If the shape is designed to be filled, the second value must be not 
    /// `None`.
    filled: (bool, Option<RGBA>),
}

impl<const N: usize> Shape<N> {
    /// Creates a shape unfilled but with borders.
    pub fn new(points: [Point; N], borders: [Border; N]) -> Self {
        Self {
            points,
            borders: Some(borders),
            filled: (false, None),
        }
    }

    /// Creates a shape filled of `colour`. Borders can be specified.
    pub fn filled(points: [Point; N], colour: RGBA, borders: Option<[Border; N]>) -> Self {
        Self {
            points,
            borders,
            filled: (true, Some(colour)),
        }
    }

    /// Returns the shape's borders if they exist, otherwise returns `None`. 
    pub fn borders(&self) -> &Option<[Border; N]> {
        &self.borders
    }

    /// Whether the shape is filled or not.
    pub fn is_filled(&self) -> bool {
        self.filled.0
    }

    /// Returns the fill colour when exists, otherwise returns `None`. But, also
    /// returns `None` if the shape is specified to not be filled, no matter if 
    /// the fill colour exists or not. 
    /// 
    /// Panic if the shape is specified to be filled and the fill colour is 
    /// `None`.
    pub fn fill_colour(&self) -> Option<RGBA> {
        if !self.is_filled() {
            // The shape is not filled, there is no fill colour or we don't care 
            // about it.
            None
        } else {
            // The shape is filled, must return the fill colour.
            if self.filled.1 == None {
                panic!(
                    "(is_filled == true) but there is no specified fill colour"
                );
            }
            
            // Returns the fill colour
            self.filled.1
        }
    }
}
