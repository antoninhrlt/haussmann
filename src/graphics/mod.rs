// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Tools to draw, to align in layouts, to shape and to colour
//! [`Widget`s](crate::Widget).

mod aligner;
pub mod colours;
mod drawer;
pub mod shapes;

pub use aligner::Aligner;
pub use drawer::Drawer;
pub use shapes::Shape;

/// Vector of type `T` with 2 values.
pub type Vec2<T> = [T; 2];

/// Point on a 2D surface of coordinates `[x, y]` being `isize` values.
pub type Point = Vec2<isize>;

/// Size of `[width, height]` being `usize` values.
pub type Size = Vec2<usize>;

/// Calculates the width and height of the shape.
pub fn calculate_size(shape: &Shape) -> Size {
    // Calculates the width of the shape.
    let mut x_values = shape
        .points()
        .into_iter()
        .map(|point| point[0])
        .collect::<Vec<isize>>();

    x_values.sort();

    let greatest_x = x_values[0];
    let smallest_x = x_values[x_values.len() - 1];

    let width = smallest_x - greatest_x;

    // Calculates the height of the shape.
    let mut y_values = shape
        .points()
        .into_iter()
        .map(|point| point[1])
        .collect::<Vec<isize>>();

    y_values.sort();

    let greatest_y = y_values[0];
    let smallest_y = y_values[y_values.len() - 1];

    let height = smallest_y - greatest_y;

    [width as usize, height as usize]
}
