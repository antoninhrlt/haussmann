// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{widgets::Layout, Align, Direction, Widget};

use super::{Shape, Point, Size};

pub struct Aligner<'a> {
    layout: &'a Layout,
    index: usize,
    /// All the widths of the shapes.
    widths: Vec<usize>,
    /// All the heights of the shapes.
    heights: Vec<usize>,
}

impl<'a> Aligner<'a> {
    /// Creates a new `Aligner` for this `layout`.
    pub fn new(layout: &'a Layout) -> Self {
        let mut obj = Self {
            layout,
            index: 0,
            widths: vec![],
            heights: vec![],
        };

        obj.update();
        obj
    }

    /// When the layout changes, this function must be called.
    ///
    /// Called by `Aligner::new()`.
    pub fn update(&mut self) {
        for widget in &self.layout.widgets {
            for shape in &widget.shapes() {
                let shape_size: Size = super::calculate_size(shape);
                
                self.widths.push(shape_size[0]);
                self.heights.push(shape_size[1]);
            }
        }
    }

    /// Returns the position of the `shape`, aligned in the layout.
    pub fn align(&mut self, shape: &Shape) -> Point {
        self.index += 1;
        
        if !self.layout.is_fixed() {
            panic!("not a fixed layout : {:?}", self.layout);
        }

        // Calculate the size of the `shape`.
        let shape_size: Size = super::calculate_size(shape);
        // Gets the size of the `layout`.
        let layout_size: Size = self.layout.size();

        // Total of widths of the shapes already placed.
        let offset_width = self.widths[..self.index - 1].iter().sum::<usize>() as isize;
        // Total of heights of the shapes already placed.
        let offset_height = self.heights[..self.index - 1].iter().sum::<usize>() as isize;
        
        // Total of widths of the shapes which are not already placed.
        let setoff_width = self.widths[self.index - 1..].iter().sum::<usize>() as isize;
        // Total of heights of the shapes which are already placed.
        let setoff_height = self.heights[self.index - 1..].iter().sum::<usize>() as isize;

        let (x, y) = match self.layout.direction {
            Direction::Column => {
                let x = match self.layout.wx_align {
                    Align::Left => offset_width,
                    Align::Center => {
                        // Both the width of the remaining space at left and right. 
                        let space: isize = (layout_size[0] as isize - self.widths.iter().sum::<usize>() as isize) / 2;

                        if self.index == 1 {
                            // First shape placed
                            space
                        } else {
                            space + offset_width
                        }
                    }
                    Align::Right => layout_size[0] as isize - setoff_width,
                    _ => panic!("layout widgets alignment on the x axis is `Align::{:?}` but should be either `Align::Left`, `Align::Center` or `Align::Right`", self.layout.wx_align),
                };

                let y = match self.layout.wy_align {
                    Align::Top => 0,
                    Align::Center => (layout_size[1] as isize - shape_size[1] as isize) / 2,
                    Align::Right => layout_size[1] as isize - shape_size[1] as isize,
                    _ => panic!("layout widgets alignment on the y axis is `Align::{:?}` but should be either `Align::Top`, `Align::Center` or `Align::Bottom`", self.layout.wy_align),
                };

                (x, y)
            }
            Direction::Row => {
                let x = match self.layout.wx_align {
                    Align::Left => 0,
                    Align::Center => (layout_size[0] as isize - shape_size[0] as isize) / 2,
                    Align::Right => layout_size[0] as isize - shape_size[0] as isize,
                    _ => panic!("layout widgets alignment on the x axis is `Align::{:?}` but should be either `Align::Left`, `Align::Center` or `Align::Right`", self.layout.wx_align),
                };

                let y = match self.layout.wy_align {
                    Align::Top => offset_height,
                    Align::Center => {
                        // Both the height of the remaining space at top and bottom. 
                        let space: isize = (layout_size[1] as isize - self.heights.iter().sum::<usize>() as isize) / 2;

                        if self.index == 1 {
                            // First shape placed
                            space
                        } else {
                            space + offset_height
                        }
                    }
                    Align::Bottom => layout_size[1] as isize - setoff_height as isize,
                    _ => panic!("layout widgets alignment on the y axis is `Align::{:?}` but should be either `Align::Top`, `Align::Center` or `Align::Bottom`", self.layout.wy_align),
                };

                (x, y)
            }
        };

        [self.layout.position()[0] + x, self.layout.position()[1] + y]
    }
}
