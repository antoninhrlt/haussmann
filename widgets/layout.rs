// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{ Overflow, Align, graphics::{Size, Point} };
use super::Widget;

/// Fixed layout with a fixed position and size, to contain several widgets.
/// 
/// Obviously used as the first app layout containing all the widgets and other 
/// layouts.
#[derive(Debug, Clone)]
pub struct FixedLayout<T: Widget> {
    /// The fixed position of the layout.
    pub position: Point,
    /// The fixed size of the layout.
    pub size: Size,
    /// The widgets contained in the layout.
    pub widgets: Vec<T>,
    /// Rules about widget overflowing.
    pub overflow: Overflow,
    /// Widgets alignment on the X axis inside the layout.
    pub wx_align: Align,
    /// Widgets alignment on the Y axis inside the layout.
    pub wy_align: Align,
}

impl<T: Widget> FixedLayout<T> {
    pub fn new(position: Point, size: Size, widgets: &[T], overflow: Overflow, wx_align: Align, wy_align: Align) -> Self {
        Self {
            size,
            position,
            widgets: widgets.to_vec(),
            overflow,
            wx_align,
            wy_align,
        }
    }
}

/// Layout to contain several widgets.
#[derive(Debug, Clone)]
pub struct Layout<T: Widget> {
    pub size: Size,
    /// The widgets contained in the layout.
    pub widgets: Vec<T>,
    /// Rules about widget overflowing.
    pub overflow: Overflow,
    /// Layout alignment on the X axis.
    pub x_align: Align,
    /// Layout alignment on the Y axis.
    pub y_align: Align, 
    /// Widgets alignment on the X axis inside the layout.
    pub wx_align: Align,
    /// Widgets alignment on the Y axis inside the layout.
    pub wy_align: Align,
}

impl<T: Widget> Layout<T> {
    pub fn new(widgets: &[T], overflow: Overflow, x_align: Align, y_align: Align, wx_align: Align, wy_align: Align) -> Self {
        Self {
            size: todo!("calculates layout size from the widgets"),
            widgets: widgets.to_vec(),
            overflow,
            x_align,
            y_align,
            wx_align,
            wy_align,
        }
    }

    /// Adds a widget to the layout.
    pub fn add_widget(&mut self, widget: T) {
        self.widgets.push(widget);
    }
}
