// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{widgets::Layout, Align, Direction};

use super::{Shape, Size};

/// Aligns shapes following the rules of a [`Layout`] widget.
#[derive(Debug)]
pub struct Aligner {
    /// All the widths of the shapes.
    widths: Vec<usize>,
    /// All the heights of the shapes.
    heights: Vec<usize>,
    /// Index of the manipulated shape.
    pub index: usize,
}

impl Aligner {
    /// Creates a new aligner calculating the size of all the shapes grouped
    /// to avoid calculating it them at each [`Self::align_shapes`] call.
    pub fn new(shapes: &Vec<Shape>) -> Self {
        let mut widths = vec![];
        let mut heights = vec![];

        for shape in &shapes.clone() {
            // This calculation creates a rectangle zone containing the whole
            // shape no matter its actual form.
            let shape_size: Size = super::calculate_size(shape);

            widths.push(shape_size[0]);
            heights.push(shape_size[1]);
        }

        Self {
            index: 0,
            widths,
            heights,
        }
    }

    /// Changes the position of `shapes` to be aligned in the layout following
    /// its rules.
    pub fn align_shapes(&mut self, layout: &Layout, layout_shape: &Shape, shapes: &mut Vec<Shape>) {
        for mut shape in shapes {
            self.align_shape(layout, layout_shape, &mut shape);
        }
    }

    /// Changes the position of a `shape` to be aligned in the layout following
    /// its rules.
    pub fn align_shape(&mut self, layout: &Layout, layout_shape: &Shape, shape: &mut Shape) {
        // Calculate the size of the `shape`.
        let shape_size: Size = super::calculate_size(shape);
        // Gets the size of the layout.
        let parent_size: Size = super::calculate_size(layout_shape);

        // Total of widths of the shapes already placed.
        let offset_width = self.widths[..self.index].iter().sum::<usize>() as isize;
        // Total of heights of the shapes already placed.
        let offset_height = self.heights[..self.index].iter().sum::<usize>() as isize;

        // Total of widths of the shapes which are not already placed.
        let setoff_width = self.widths[self.index..].iter().sum::<usize>() as isize;
        // Total of heights of the shapes which are already placed.
        let setoff_height = self.heights[self.index..].iter().sum::<usize>() as isize;

        let (x, y) = match layout.direction {
            Direction::Column => (
                match layout.wx_align {
                    Align::Left => offset_width,
                    Align::Center => {
                        // Both the width of the remaining space at left and right. 
                        let space: isize = (parent_size[0] as isize - self.widths.iter().sum::<usize>() as isize) / 2;

                        if self.index == 0 {
                            // First shape placed
                            space
                        } else {
                            space + offset_width
                        }
                    }
                    Align::Right => parent_size[0] as isize - setoff_width,
                    _ => panic!("layout widgets alignment on the x axis is `Align::{:?}` but should be either `Align::Left`, `Align::Center` or `Align::Right`", layout.wx_align),
                },
                match layout.wy_align {
                    Align::Top => 0,
                    Align::Center => (parent_size[1] as isize - shape_size[1] as isize) / 2,
                    Align::Bottom => parent_size[1] as isize - shape_size[1] as isize,
                    _ => panic!("layout widgets alignment on the y axis is `Align::{:?}` but should be either `Align::Top`, `Align::Center` or `Align::Bottom`", layout.wy_align),
                }
            ),
            Direction::Row => (
                match layout.wx_align {
                    Align::Left => 0,
                    Align::Center => (parent_size[0] as isize - shape_size[0] as isize) / 2,
                    Align::Right => parent_size[0] as isize - shape_size[0] as isize,
                    _ => panic!("layout widgets alignment on the x axis is `Align::{:?}` but should be either `Align::Left`, `Align::Center` or `Align::Right`", layout.wx_align),
                },
                match layout.wy_align {
                    Align::Top => offset_height,
                    Align::Center => {
                        // Both the height of the remaining space at top and bottom. 
                        let space: isize = (parent_size[1] as isize - self.heights.iter().sum::<usize>() as isize) / 2;

                        if self.index == 0 {
                            // First shape placed
                            space
                        } else {
                            space + offset_height
                        }
                    }
                    Align::Bottom => parent_size[1] as isize - setoff_height as isize,
                    _ => panic!("layout widgets alignment on the y axis is `Align::{:?}` but should be either `Align::Top`, `Align::Center` or `Align::Bottom`", layout.wy_align),
                }
            )
        };

        let layout_position = layout_shape.points()[0];
        shape.move_by([layout_position[0] + x, layout_position[1] + y]);
        self.index += 1;
    }
}
