// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

pub mod shapes;
pub mod colours;

pub use shapes::Shape;

use crate::{widgets::{Layout, Widget, FixedLayout}, Align};

/// Vector of type `T` with 2 values.
pub type Vec2<T> = [T; 2];

/// Point on a 2D surface of coordinates `[x, y]` being `isize` values.
pub type Point = Vec2<isize>;

/// Size of `[width, height]` being `usize` values.
pub type Size = Vec2<usize>;

/// Calculates the width and height of the shape.
pub fn calculate_size(shape: &Shape) -> (isize, isize) {
    // Calculates the width of the shape.
    let mut x_values = shape
        .points()
        .into_iter()
        .map(|point| point[0])
        .collect::<Vec<isize>>();

    x_values.sort();

    let greatest_x = x_values[0];
    let smallest_x = x_values[x_values.len() -1];

    let width = smallest_x - greatest_x;

    // Calculates the height of the shape.
    let mut y_values = shape
        .points()
        .into_iter()
        .map(|point| point[1])
        .collect::<Vec<isize>>();

    y_values.sort();

    let greatest_y = y_values[0];
    let smallest_y = y_values[y_values.len() -1];

    let height = smallest_y - greatest_y;

    println!("width: {width}, height: {height}");

    (width, height)
}

/// Returns the (X, Y) position of the shape according to where it should be 
/// placed from the layout containing it.
pub fn place_in_layout<T: Widget>(layout: &Layout<T>, shape: &Shape) -> (isize, isize) {
    let cast = FixedLayout::new(
        [0, 0],
        layout.size,
        layout.widgets.as_slice(),
        layout.overflow.clone(),
        layout.wx_align.clone(),
        layout.wy_align.clone(),
    );

    place_in_fixed_layout(&cast, shape)
}

/// Returns the (X, Y) position of the shape according to where it should be 
/// placed from the fixed layout containing it.
pub fn place_in_fixed_layout<T: Widget>(layout: &FixedLayout<T>, shape: &Shape) -> (isize, isize) {
    // Calculates the size of the `shape`.
    let (width, height) = calculate_size(shape);

    // Calculates the position of the shape in the layout.
    (
        match layout.wx_align {
            Align::Center => {
                layout.position[0] + (layout.size[0] as isize - width) / 2
            }
            _ => todo!()
        },
        match layout.wy_align {
            Align::Center => {
                layout.position[1] + (layout.size[1] as isize - height) / 2
            }
            _ => todo!()
        }
    )
}
