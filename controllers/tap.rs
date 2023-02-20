// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{Widget, graphics::{Size, Shape}, ToAny, DebugWidget};

#[derive(Debug)]
pub struct Detector {
    /// Widget where the tap detection will be done.
    widget: Box<dyn Widget>,
    /// Function called when `widget` is tapped.
    pub on_tap: fn(),
}

crate::dynamic_widget!(Detector);

impl Widget for Detector {
    fn shapes(&self) -> Vec<Shape> {
        self.widget.shapes()
    }

    fn size(&self) -> Size {
        self.widget.size()
    }
}

impl Detector {
    pub fn new<T: Widget + 'static>(widget: T, on_tap: fn()) -> Self {
        Self {
            widget: Box::new(widget),
            on_tap,
        }
    }

    /// Returns the zone (which is the `widget`'s size) covered by the tap 
    /// detector.
    pub fn tap_zone(&self) -> Size {
        self.widget.size()
    }
}
