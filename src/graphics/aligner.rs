// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{widgets::Layout, Direction, Align};

use super::{Point, Size};

/// Generates a vector of [`Point`] which are the positions of every widget 
/// contained in a [`Layout`].
/// 
/// Does not calculate the positions of the sub-widgets (widgets of the 
/// layout's widgets) !
#[derive(Debug)]
pub struct Aligner<'a> {
    layout: &'a Layout,
    sizes: Vec<Size>,
}

impl<'a> Aligner<'a> {
    /// Creates a new aligner following the rules of a layout.
    /// 
    /// The `sizes` parameter is the sizes of every widget calculated thanks to
    /// [`Sizer`](super::Sizer)).
    pub fn new(layout: &'a Layout, sizes: Vec<Size>) -> Self {
        Self {
            layout,
            sizes,
        }
    }

    /// Gets the position of each widget in the correct order, in a positioned zone.
    pub fn align_at(&self, zone: Point) -> Vec<Point> {
        // No size given, no alignment has to be done.
        if self.sizes.is_empty() {
            return vec![];
        }

        // The first size is always the layout's size.
        let zone_size: Size = self.sizes[0];

        let widths: Vec<usize> = self.sizes[1..]
            .iter()
            .map(|size| size[0])
            .collect();
        
        let heights: Vec<usize> = self.sizes[1..]
            .iter()
            .map(|size| size[1])
            .collect();

        // Creates the positions in the correct order.
        // The first position is the layout's position, which is the position of
        // the zone.
        let mut positions: Vec<Point> = vec![zone];

        for (i, size) in self.sizes[1..].iter().enumerate() {
            // Total of widths of the widgets already placed.
            let offset_width = widths[..i].iter().sum::<usize>() as isize;
            // Total of heights of the widgets already placed.
            let offset_height = heights[..i].iter().sum::<usize>() as isize;

            // Total of widths of the widgets which are not already placed.
            let setoff_width = widths[i..].iter().sum::<usize>() as isize;
            // Total of heights of the widgets which are already placed.
            let setoff_height = heights[i..].iter().sum::<usize>() as isize;

            let (x, y) = match self.layout.direction {
                Direction::Column => (
                    match self.layout.wx_align {
                        Align::Left => offset_width,
                        Align::Center => {
                            // Both the width of the remaining space at left and right. 
                            let space: isize = (zone_size[0] as isize - widths.iter().sum::<usize>() as isize) / 2;

                            if i == 0 {
                                // First widget placed
                                space
                            } else {
                                space + offset_width
                            }
                        }
                        Align::Right => zone_size[0] as isize - setoff_width,
                        _ => panic!("layout widgets alignment on the x axis is `Align::{:?}` but should be either `Align::Left`, `Align::Center` or `Align::Right`", self.layout.wx_align),
                    },
                    match self.layout.wy_align {
                        Align::Top => 0,
                        Align::Center => (zone_size[1] as isize - size[1] as isize) / 2,
                        Align::Bottom => zone_size[1] as isize - size[1] as isize,
                        _ => panic!("layout widgets alignment on the y axis is `Align::{:?}` but should be either `Align::Top`, `Align::Center` or `Align::Bottom`", self.layout.wy_align),
                    }
                ),
                Direction::Row => (
                    match self.layout.wx_align {
                        Align::Left => 0,
                        Align::Center => (zone_size[0] as isize - size[0] as isize) / 2,
                        Align::Right => zone_size[0] as isize - size[0] as isize,
                        _ => panic!("layout widgets alignment on the x axis is `Align::{:?}` but should be either `Align::Left`, `Align::Center` or `Align::Right`", self.layout.wx_align),
                    },
                    match self.layout.wy_align {
                        Align::Top => offset_height,
                        Align::Center => {
                            // Both the height of the remaining space at top and bottom. 
                            let space: isize = (zone_size[1] as isize - heights.iter().sum::<usize>() as isize) / 2;

                            if i == 0 {
                                // First widget placed
                                space
                            } else {
                                space + offset_height
                            }
                        }
                        Align::Bottom => zone_size[1] as isize - setoff_height as isize,
                        _ => panic!("layout widgets alignment on the y axis is `Align::{:?}` but should be either `Align::Top`, `Align::Center` or `Align::Bottom`", self.layout.wy_align),
                    }
                )
            };

            positions.push([zone[0] + x, zone[1] + y]);
        }

        positions
    }
}
