// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to tapping widgets.

use crate::{
    graphics::{Shape, Size, Point},
    DebugWidget, ToAny, Widget,
};

/// Controller wrapping a widget to detect when it is tapped.
#[derive(Debug)]
pub struct Detector<'a, T: Widget> {
    /// Widget where the tap detection will be done.
    pub widget: Box<T>,
    /// Function called when the widget is tapped.
    pub on_tap: fn(widget: &'a mut T),
}

crate::dynamic_controller!(Detector<'a, T>);

impl<'a, T: Widget + 'static> Widget for Detector<'a, T> {
    /// Calls `Widget::shape()` on `self.widget` and returns the returned value
    /// of this function.
    fn shape(&self, position: Option<Point>, size: Option<Size>) -> Shape {
        self.widget.shape(position, size)
    }
}

impl<'a, T: Widget> Detector<'a, T> {
    /// Creates a new detector for a widget calling `on_tap` when it is tapped.
    pub fn new(
        widget: T,
        on_tap: fn(widget: &'a mut T),
    ) -> Self {
        Self {
            widget: Box::new(widget),
            on_tap,
        }
    }

    /// Function to call when the widget is tapped.
    pub fn on_tap(&'static mut self) {
        let call = self.on_tap;
        call(&mut self.widget);
    }
}
