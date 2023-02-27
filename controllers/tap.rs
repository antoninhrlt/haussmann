// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to tapping widgets.

use crate::{
    graphics::{Shape, Size, Point},
    DebugWidget, ToAny, Widget,
};

// See here, however you need to handle lifetimes lol
pub type OnTap<T> = fn(widget: &'static mut T);

/// Controller wrapping a widget to detect when it is tapped.
#[derive(Debug)]
pub struct Detector<T: Widget + 'static> {
    /// Widget where the tap detection will be done.
    pub widget: Box<T>,
    /// Function called when the widget is tapped.
    pub on_tap: OnTap<T>,
    /// The zone covered by the detector, which is supposed to be the position 
    /// and size of the widget.
    pub zone: Option<(Point, Size)>,
}

crate::dynamic_controller!(Detector<T>);

impl<T: Widget + 'static> Widget for Detector<T> {
    /// Calls `Widget::shape()` on `self.widget` and returns the returned value
    /// of this function.
    fn shape(&self, position: Option<Point>, size: Option<Size>) -> Shape {
        self.widget.shape(position, size)
    }
}

impl<T: Widget> Detector<T> {
    /// Creates a new detector for a widget calling `on_tap` when it is tapped.
    pub fn new(
        widget: T,
        on_tap: OnTap<T>,
    ) -> Self {
        Self {
            widget: Box::new(widget),
            on_tap,
            zone: None,
        }
    }

    /// Function to call when the widget is tapped.
    pub fn on_tap(&'static mut self) {
        let call = self.on_tap;
        call(&mut self.widget);
    }
}
