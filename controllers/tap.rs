// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{Widget, graphics::{Size, Shape}, ToAny, DebugWidget};

#[derive(Debug)]
pub struct Detector {
    /// Widget where the tap detection will be done.
    pub widget: Box<dyn Widget>,
    /// Function called when `widget` is tapped.
    pub on_tap: fn(),
}

crate::dynamic_widget!(Detector);

impl Widget for Detector {
    /// Calls `Widget::shape()` on `self.widget` and returns the returned value 
    /// of this function.
    fn shape(&self, size: Size) -> Shape {
        self.widget.shape(size)
    }
}

impl Detector {
    pub fn new<T: Widget + 'static>(widget: T, on_tap: fn()) -> Self {
        Self {
            widget: Box::new(widget),
            on_tap,
        }
    }
}
