// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use super::Shape;
use super::Point;
use crate::{Border, Radius};
use crate::graphics::colours::RGBA;

/// Creates a `Shape` with `N` points and `N` borders.
/// 
/// ## Why a shape builder 
/// The purpose of this builder is to create a safe `Shape` object. Indeed, if 
/// the shape were proposing build functions itself, the user could define a 
/// structure with a different number of points than the borders or create a 
/// filled shape without a fill colour.
/// 
/// > But, why not just using the const template argument `N` with `Shape` ?
/// 
/// The `Shape` structure wants to be a sort of parent for all the type of 
/// shapes. Adding a `N` const template argument would not permit to have 
/// different sort of shapes in a same widget. 
pub struct Builder<const N: usize> {
    shape: Option<Shape>,
}

/// Implementation for any type of shape.
impl<const N: usize> Builder<N> {
    /// Creates a new `shapes::Builder` object. The `shape` is `None`.
    pub fn new() -> Self {
        Self { shape: None }
    }

    /// Creates the shape with its `points` and `borders`.
    pub fn create(&mut self, points: [Point; N], borders: Option<[Border; N]>) -> &mut Self {
        // If there is no border specified, gives an empty vector.
        let borders = if borders == None {
            vec![] 
        } else { 
            borders.unwrap().to_vec() 
        };
        
        self.shape = Some(
            Shape {
                points: points.to_vec(),
                borders,
                filled: (false, None),
                radius: Radius::new(0.0),
            }
        );

        self
    }

    /// Fills the shape.
    pub fn fill(&mut self, colour: RGBA) -> &mut Self {
        self.shape.as_mut().unwrap().filled = (true, Some(colour));
        self
    }

    /// Rounds the shape with `radius`.
    pub fn round(&mut self, radius: Radius) -> &mut Self {
        self.shape.as_mut().unwrap().radius = radius;
        self
    }

    /// Returns the shape finished.
    pub fn finish(&mut self) -> Shape {
        self.shape.clone().unwrap()
    }
}
