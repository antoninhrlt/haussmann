// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to tapping widgets.

use crate::{
    graphics::{Shape, Size},
    DebugWidget, ToAny, Widget,
};

/// Controller wrapping a widget to detect when it is tapped.
#[derive(Debug)]
pub struct Detector {
    /// Widget where the tap detection will be done.
    pub widget: Box<dyn Widget>,
    /// Function called when the widget is tapped.
    pub on_tap: fn(widget: &'static mut Box<dyn Widget>),
}

crate::dynamic_widget!(Detector);

impl Widget for Detector {
    /// Calls `Widget::shape()` on `self.widget` and returns the returned value
    /// of this function.
    fn shape(&self, size: Option<Size>) -> Shape {
        self.widget.shape(size)
    }
}

impl Detector {
    /// Creates a new detector for a widget calling `on_tap` when it is tapped.
    pub fn new<T: Widget + 'static>(
        widget: T,
        on_tap: fn(widget: &'static mut Box<dyn Widget>),
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
