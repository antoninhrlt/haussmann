// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

pub mod shapes;
pub mod colours;

pub use shapes::Shape;

use crate::{widgets::Layout, Widget, Align, direction, Direction};

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

    [width as usize, height as usize]
}

/// Returns the (X, Y) position of the shape according to where it should be 
/// placed from the fixed layout containing it.
/// 
/// The layout must have a non-`None` `position` field: it must be a "fixed" 
/// layout. Otherwise, this function panics.
pub fn place_in_fixed_layout(layout: &Layout, shape: &Shape, index: usize) -> Point {
    if !layout.is_fixed() {
        panic!("not a fixed layout : {:?}", layout);
    }

    // Calculates the size of the `shape`.
    let size: Size = calculate_size(shape);

    // Total of widths of the already-placed shapes.
    let mut offset_w_placed = 0;

    for widget in &layout.widgets[..index + 1] {
        for shape in &widget.shapes() {
            offset_w_placed += calculate_size(shape)[0];
        }
    }

    // Total of heights of the already-placed shapes.
    let mut offset_h_placed = 0;

    for widget in &layout.widgets[..index + 1] {
        for shape in &widget.shapes() {
            offset_h_placed += calculate_size(shape)[1];
        }
    }

    let mut x: isize = 0;
    let mut y: isize = 0;

    match layout.direction {
        Direction::Column => {
            x = match layout.wx_align {
                Align::Left => offset_w_placed as isize,
                Align::Center => {
                    let mut total = 0;
                        
                    for widget in &layout.widgets {
                        for shape in widget.shapes() {
                            total += calculate_size(&shape)[0];
                        }
                    }

                    if index == 0 {
                        (layout.size()[0] - total) as isize / 2
                    } else {
                        (layout.size()[0] - total) as isize / 2 + offset_w_placed as isize
                    }
                }
                Align::Right => {
                    // Total of widths of the shapes which are not already placed.
                    let mut offset = 0;

                    for widget in &layout.widgets[index + 1..] {
                        for shape in widget.shapes() {
                            offset += calculate_size(&shape)[0];
                        }
                    }
                    
                    layout.size()[0] as isize - offset as isize
                },
                _ => panic!("layout widgets alignment on the x axis is `Align::{:?}` but should be either `Align::Left`, `Align::Center` or `Align::Right`", layout.wx_align),
            };

            y = match layout.wy_align {
                Align::Top => 0,
                Align::Center => {
                    (layout.size()[1] - size[1]) as isize / 2
                }
                Align::Bottom => layout.size()[1] as isize - size[1] as isize,
                _ => panic!("layout widgets alignment on the y axis is `Align::{:?}` but should be either `Align::Top`, `Align::Center` or `Align::Bottom`", layout.wy_align),
            };
        }
        Direction::Row => {
            x = match layout.wx_align {
                Align::Left => 0,
                Align::Center => {
                    (layout.size()[0] - size[0]) as isize / 2
                }
                Align::Right => {
                    layout.size()[0] as isize - size[0] as isize
                },
                _ => panic!("layout widgets alignment on the x axis is `Align::{:?}` but should be either `Align::Left`, `Align::Center` or `Align::Right`", layout.wy_align),
            };

            y = match layout.wy_align {
                Align::Top => {
                    offset_h_placed as isize
                }
                Align::Center => {
                    let mut total = 0;
                        
                    for widget in &layout.widgets {
                        for shape in &widget.shapes() {
                            total += calculate_size(shape)[1];
                        }
                    }

                    if index == 0 {
                        (layout.size()[1] - total) as isize / 2
                    } else {
                        (layout.size()[1] - total) as isize / 2 + offset_h_placed as isize
                    }
                }
                Align::Bottom => {
                    // Total of heights of the shapes which are not already placed.
                    let mut offset = 0;

                    for widget in &layout.widgets[index + 1..] {
                        for shape in &widget.shapes() {
                            offset += calculate_size(shape)[1];
                        }
                    }
                    
                    layout.size()[1] as isize - offset as isize                }
                _ => panic!("layout widgets alignment on the y axis is `Align::{:?}` but should be either `Align::Top`, `Align::Center` or `Align::Bottom`", layout.wx_align),
            };
        }
    }
    
    [layout.position()[0] + x, layout.position()[1] + y]
}
