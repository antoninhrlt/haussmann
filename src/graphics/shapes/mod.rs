// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use super::{ Size, Point };
use crate::Border;
use super::colours::RGBA;

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
pub struct ShapeBuilder<const N: usize> {
    shape: Option<Shape>,
}

impl<const N: usize> ShapeBuilder<N> {
    /// Creates a new `ShapeBuilder` object. The `shape` is `None`.
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
            }
        );

        self
    }

    /// Fills the shape.
    pub fn fill(&mut self, colour: RGBA) -> &mut Self {
        self.shape.as_mut().unwrap().filled = (true, Some(colour));
        self
    }

    /// Returns the shape finished.
    pub fn finish(&mut self) -> Shape {
        self.shape.clone().unwrap()
    }
}

/// The parent of any sort of shape. To create a `Shape` object, see 
/// `ShapeBuilder`.
/// 
/// There is the same number of `points` than `borders`. The shape cannot be set 
/// as `filled` without fill colour.
/// 
/// Thanks to `ShapeBuilder` it's surely a safe structure, there cannot be any 
/// illogical problem.
#[derive(Debug, Clone)]
pub struct Shape {
    /// The points of the shape.
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
}

impl Shape {
    /// Returns the shape's points.
    pub fn points(&self) -> &Vec<Point> {
        &self.points
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
}
